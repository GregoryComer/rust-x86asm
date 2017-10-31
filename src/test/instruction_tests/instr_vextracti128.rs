use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vextracti128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI128, operand1: Some(Direct(XMM5)), operand2: Some(Direct(YMM4)), operand3: Some(Literal8(67)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 57, 229, 67], OperandSize::Dword)
}

#[test]
fn vextracti128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI128, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Eight, 1167941305, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(96)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 57, 132, 216, 185, 94, 157, 69, 96], OperandSize::Dword)
}

#[test]
fn vextracti128_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI128, operand1: Some(Direct(XMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(70)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 57, 221, 70], OperandSize::Qword)
}

#[test]
fn vextracti128_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTI128, operand1: Some(IndirectDisplaced(RDX, 545176606, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(81)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 57, 130, 30, 188, 126, 32, 81], OperandSize::Qword)
}

