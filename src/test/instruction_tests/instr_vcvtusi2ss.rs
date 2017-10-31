use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtusi2ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(EDI)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 94, 24, 123, 207], OperandSize::Dword)
}

#[test]
fn vcvtusi2ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 94, 8, 123, 4, 193], OperandSize::Dword)
}

#[test]
fn vcvtusi2ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(ESP)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 14, 120, 123, 204], OperandSize::Qword)
}

#[test]
fn vcvtusi2ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SS, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Four, 1485051628, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 54, 8, 123, 140, 144, 236, 22, 132, 88], OperandSize::Qword)
}

#[test]
fn vcvtusi2ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SS, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(RDI)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 198, 48, 123, 199], OperandSize::Qword)
}

#[test]
fn vcvtusi2ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUSI2SS, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 174, 8, 123, 60, 72], OperandSize::Qword)
}

