use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vextracti128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI128, operand1: Some(Direct(XMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(25)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 57, 255, 25], OperandSize::Dword)
}

#[test]
fn vextracti128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI128, operand1: Some(IndirectDisplaced(ECX, 1303390937, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(59)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 57, 169, 217, 42, 176, 77, 59], OperandSize::Dword)
}

#[test]
fn vextracti128_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI128, operand1: Some(Direct(XMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 57, 202, 9], OperandSize::Qword)
}

#[test]
fn vextracti128_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI128, operand1: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM2)), operand3: Some(Literal8(48)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 57, 16, 48], OperandSize::Qword)
}

