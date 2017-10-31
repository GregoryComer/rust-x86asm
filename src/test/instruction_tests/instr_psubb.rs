use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psubb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(MM5)), operand2: Some(Direct(MM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 248, 238], OperandSize::Dword)
}

#[test]
fn psubb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(MM3)), operand2: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Two, 1971904049, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 248, 156, 123, 49, 222, 136, 117], OperandSize::Dword)
}

#[test]
fn psubb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 248, 220], OperandSize::Qword)
}

#[test]
fn psubb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(MM2)), operand2: Some(IndirectDisplaced(RDX, 393785882, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 248, 146, 26, 178, 120, 23], OperandSize::Qword)
}

#[test]
fn psubb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 248, 192], OperandSize::Dword)
}

#[test]
fn psubb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Two, 1920266186, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 248, 180, 95, 202, 239, 116, 114], OperandSize::Dword)
}

#[test]
fn psubb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 248, 195], OperandSize::Qword)
}

#[test]
fn psubb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSUBB, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 248, 33], OperandSize::Qword)
}

