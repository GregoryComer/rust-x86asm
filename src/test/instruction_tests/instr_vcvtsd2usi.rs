use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtsd2usi_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2USI, operand1: Some(Direct(EBX)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 120, 121, 223], OperandSize::Dword)
}

#[test]
fn vcvtsd2usi_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2USI, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Four, 982804664, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 8, 121, 164, 153, 184, 104, 148, 58], OperandSize::Dword)
}

#[test]
fn vcvtsd2usi_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2USI, operand1: Some(Direct(ESP)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 56, 121, 227], OperandSize::Qword)
}

#[test]
fn vcvtsd2usi_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2USI, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 684686232, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 8, 121, 12, 189, 152, 123, 207, 40], OperandSize::Qword)
}

#[test]
fn vcvtsd2usi_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2USI, operand1: Some(Direct(RDI)), operand2: Some(Direct(XMM8)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 209, 255, 120, 121, 248], OperandSize::Qword)
}

#[test]
fn vcvtsd2usi_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2USI, operand1: Some(Direct(RDI)), operand2: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 255, 8, 121, 58], OperandSize::Qword)
}

