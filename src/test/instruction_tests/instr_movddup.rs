use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movddup_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDDUP, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 18, 233], OperandSize::Dword)
}

#[test]
fn movddup_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDDUP, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 18, 7], OperandSize::Dword)
}

#[test]
fn movddup_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDDUP, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 18, 254], OperandSize::Qword)
}

#[test]
fn movddup_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVDDUP, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 825051105, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 18, 36, 117, 225, 71, 45, 49], OperandSize::Qword)
}

