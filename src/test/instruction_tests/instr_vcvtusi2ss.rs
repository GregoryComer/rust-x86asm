use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtusi2ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(ESP)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 110, 56, 123, 196], OperandSize::Dword)
}

#[test]
fn vcvtusi2ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(ECX, EDI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 102, 8, 123, 12, 185], OperandSize::Dword)
}

#[test]
fn vcvtusi2ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SS, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(ESI)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 22, 56, 123, 254], OperandSize::Qword)
}

#[test]
fn vcvtusi2ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Eight, 94793801, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 14, 0, 123, 156, 192, 73, 112, 166, 5], OperandSize::Qword)
}

#[test]
fn vcvtusi2ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SS, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM24)), operand3: Some(Direct(RBX)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 190, 80, 123, 219], OperandSize::Qword)
}

#[test]
fn vcvtusi2ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectDisplaced(RCX, 397595801, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 150, 8, 123, 153, 153, 212, 178, 23], OperandSize::Qword)
}

