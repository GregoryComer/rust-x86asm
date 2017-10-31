use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn movntpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTPD, operand1: Some(IndirectDisplaced(ESI, 1376363666, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 43, 190, 146, 164, 9, 82], OperandSize::Dword)
}

#[test]
fn movntpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVNTPD, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Eight, 585280777, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 43, 172, 209, 9, 173, 226, 34], OperandSize::Qword)
}

