use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vextractf128_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF128, operand1: Some(Direct(XMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Literal8(28)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 25, 240, 28], OperandSize::Dword)
}

#[test]
fn vextractf128_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF128, operand1: Some(IndirectScaledIndexed(EAX, ESI, Eight, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(8)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 25, 4, 240, 8], OperandSize::Dword)
}

#[test]
fn vextractf128_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF128, operand1: Some(Direct(XMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(79)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 25, 236, 79], OperandSize::Qword)
}

#[test]
fn vextractf128_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXTRACTF128, operand1: Some(IndirectScaledIndexed(RAX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(59)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 25, 28, 200, 59], OperandSize::Qword)
}

