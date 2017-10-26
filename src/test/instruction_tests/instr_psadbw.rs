use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn psadbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(MM1)), operand2: Some(Direct(MM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 246, 200], OperandSize::Dword)
}

#[test]
fn psadbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(MM2)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 1980351693, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 246, 20, 205, 205, 196, 9, 118], OperandSize::Dword)
}

#[test]
fn psadbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(MM3)), operand2: Some(Direct(MM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 246, 217], OperandSize::Qword)
}

#[test]
fn psadbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(MM7)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 1365113216, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 246, 60, 189, 128, 249, 93, 81], OperandSize::Qword)
}

#[test]
fn psadbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 246, 219], OperandSize::Dword)
}

#[test]
fn psadbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Two, 615573743, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 246, 172, 118, 239, 232, 176, 36], OperandSize::Dword)
}

#[test]
fn psadbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 246, 203], OperandSize::Qword)
}

#[test]
fn psadbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::PSADBW, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(RBX, 1374524460, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 246, 171, 44, 148, 237, 81], OperandSize::Qword)
}

