use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vcvtsd2si_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(ESP)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 45, 230], OperandSize::Dword)
}

#[test]
fn vcvtsd2si_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 224255473, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 45, 52, 253, 241, 221, 93, 13], OperandSize::Dword)
}

#[test]
fn vcvtsd2si_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(EBP)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 45, 236], OperandSize::Qword)
}

#[test]
fn vcvtsd2si_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(ESP)), operand2: Some(IndirectDisplaced(RAX, 1522277089, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 45, 160, 225, 26, 188, 90], OperandSize::Qword)
}

#[test]
fn vcvtsd2si_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(RBP)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 45, 236], OperandSize::Qword)
}

#[test]
fn vcvtsd2si_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 256248554, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 45, 20, 149, 234, 10, 70, 15], OperandSize::Qword)
}

#[test]
fn vcvtsd2si_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(EDI)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 120, 45, 249], OperandSize::Dword)
}

#[test]
fn vcvtsd2si_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(ESI)), operand2: Some(IndirectDisplaced(EDI, 1506431602, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 45, 183, 114, 82, 202, 89], OperandSize::Dword)
}

#[test]
fn vcvtsd2si_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(ESI)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 88, 45, 243], OperandSize::Qword)
}

#[test]
fn vcvtsd2si_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(RDX, 2142530466, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 45, 154, 162, 107, 180, 127], OperandSize::Qword)
}

#[test]
fn vcvtsd2si_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(RBP)), operand2: Some(Direct(XMM25)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 145, 255, 88, 45, 233], OperandSize::Qword)
}

#[test]
fn vcvtsd2si_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTSD2SI, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledIndexed(RDI, RDX, Four, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 225, 251, 45, 20, 151], OperandSize::Qword)
}

