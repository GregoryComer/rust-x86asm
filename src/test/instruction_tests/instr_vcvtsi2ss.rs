use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtsi2ss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(EBP)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 42, 197], OperandSize::Dword)
}

#[test]
fn vcvtsi2ss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(EAX, EDI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 226, 42, 4, 248], OperandSize::Dword)
}

#[test]
fn vcvtsi2ss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(ECX)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 234, 42, 193], OperandSize::Qword)
}

#[test]
fn vcvtsi2ss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Two, 156003193, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 194, 42, 156, 83, 121, 107, 76, 9], OperandSize::Qword)
}

#[test]
fn vcvtsi2ss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(RSI)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 250, 42, 238], OperandSize::Qword)
}

#[test]
fn vcvtsi2ss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 250, 42, 44, 248], OperandSize::Qword)
}

#[test]
fn vcvtsi2ss_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(ESI)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 70, 24, 42, 198], OperandSize::Dword)
}

#[test]
fn vcvtsi2ss_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1430401522, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 210, 42, 52, 253, 242, 49, 66, 85], OperandSize::Dword)
}

#[test]
fn vcvtsi2ss_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM22)), operand3: Some(Direct(EBX)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 78, 48, 42, 195], OperandSize::Qword)
}

#[test]
fn vcvtsi2ss_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 46, 0, 42, 4, 208], OperandSize::Qword)
}

#[test]
fn vcvtsi2ss_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(RCX)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 182, 56, 42, 193], OperandSize::Qword)
}

#[test]
fn vcvtsi2ss_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSI2SS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectDisplaced(RCX, 603656441, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 222, 0, 42, 145, 249, 16, 251, 35], OperandSize::Qword)
}

