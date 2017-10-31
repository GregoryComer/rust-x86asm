use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvttss2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(EDI)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 44, 250], OperandSize::Dword)
}

#[test]
fn vcvttss2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexed(EDX, EAX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 44, 36, 66], OperandSize::Dword)
}

#[test]
fn vcvttss2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(ESI)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 44, 242], OperandSize::Qword)
}

#[test]
fn vcvttss2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 768853748, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 44, 60, 117, 244, 198, 211, 45], OperandSize::Qword)
}

#[test]
fn vcvttss2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(RCX)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 250, 44, 200], OperandSize::Qword)
}

#[test]
fn vcvttss2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(RBX)), operand2: Some(IndirectDisplaced(RDI, 1459352901, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 250, 44, 159, 69, 245, 251, 86], OperandSize::Qword)
}

#[test]
fn vcvttss2si_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(EBX)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 241, 126, 24, 44, 223], OperandSize::Dword)
}

#[test]
fn vcvttss2si_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 1876457298, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 44, 36, 77, 82, 119, 216, 111], OperandSize::Dword)
}

#[test]
fn vcvttss2si_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(EDI)), operand2: Some(Direct(XMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 177, 126, 24, 44, 254], OperandSize::Qword)
}

#[test]
fn vcvttss2si_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledDisplaced(RAX, Two, 655223427, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 44, 60, 69, 131, 234, 13, 39], OperandSize::Qword)
}

#[test]
fn vcvttss2si_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(RDX)), operand2: Some(Direct(XMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: true, mask: None, broadcast: None }, &[98, 177, 254, 24, 44, 210], OperandSize::Qword)
}

#[test]
fn vcvttss2si_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTSS2SI, operand1: Some(Direct(RBX)), operand2: Some(IndirectDisplaced(RAX, 858370277, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 250, 44, 152, 229, 176, 41, 51], OperandSize::Qword)
}

