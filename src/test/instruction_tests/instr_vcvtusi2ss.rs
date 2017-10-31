use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtusi2ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(ESP)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 118, 24, 123, 204], OperandSize::Dword)
}

#[test]
fn vcvtusi2ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 78, 8, 123, 20, 88], OperandSize::Dword)
}

#[test]
fn vcvtusi2ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SS, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM17)), operand3: Some(Direct(ESP)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 118, 80, 123, 228], OperandSize::Qword)
}

#[test]
fn vcvtusi2ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Four, 1791179553, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 62, 8, 123, 164, 151, 33, 59, 195, 106], OperandSize::Qword)
}

#[test]
fn vcvtusi2ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SS, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(RDX)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 238, 56, 123, 226], OperandSize::Qword)
}

#[test]
fn vcvtusi2ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SS, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 214, 8, 123, 25], OperandSize::Qword)
}

