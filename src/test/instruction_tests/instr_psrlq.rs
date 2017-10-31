use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psrlq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(MM1)), operand2: Some(Literal8(89)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 115, 209, 89], OperandSize::Dword)
}

#[test]
fn psrlq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(MM6)), operand2: Some(Literal8(25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 115, 214, 25], OperandSize::Qword)
}

#[test]
fn psrlq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(XMM4)), operand2: Some(Literal8(63)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 115, 212, 63], OperandSize::Dword)
}

#[test]
fn psrlq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(XMM5)), operand2: Some(Literal8(28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 115, 213, 28], OperandSize::Qword)
}

#[test]
fn psrlq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(MM4)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 211, 225], OperandSize::Dword)
}

#[test]
fn psrlq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(MM1)), operand2: Some(IndirectScaledDisplaced(EBX, Two, 544089023, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 211, 12, 93, 191, 35, 110, 32], OperandSize::Dword)
}

#[test]
fn psrlq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 211, 205], OperandSize::Qword)
}

#[test]
fn psrlq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(MM6)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 211, 51], OperandSize::Qword)
}

#[test]
fn psrlq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 211, 254], OperandSize::Dword)
}

#[test]
fn psrlq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Four, 138975681, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 211, 140, 184, 193, 153, 72, 8], OperandSize::Dword)
}

#[test]
fn psrlq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 211, 239], OperandSize::Qword)
}

#[test]
fn psrlq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::PSRLQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Two, 984797054, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 211, 164, 113, 126, 207, 178, 58], OperandSize::Qword)
}

