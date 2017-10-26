use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movntpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTPD, operand1: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 43, 15], OperandSize::Dword)
}

#[test]
fn movntpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTPD, operand1: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 43, 59], OperandSize::Qword)
}

