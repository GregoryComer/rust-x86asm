use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovntpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectScaledIndexed(EDI, ESI, Eight, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 43, 12, 247], OperandSize::Dword)
}

#[test]
fn vmovntpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectScaledDisplaced(RBX, Eight, 296562471, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 43, 20, 221, 39, 47, 173, 17], OperandSize::Qword)
}

#[test]
fn vmovntpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectScaledDisplaced(ECX, Two, 771138339, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 43, 36, 77, 35, 163, 246, 45], OperandSize::Dword)
}

#[test]
fn vmovntpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectDisplaced(RDX, 1959167049, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 43, 130, 73, 132, 198, 116], OperandSize::Qword)
}

#[test]
fn vmovntpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Two, 454293487, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 43, 172, 91, 239, 247, 19, 27], OperandSize::Dword)
}

#[test]
fn vmovntpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectScaledDisplaced(RAX, Four, 727845082, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 121, 43, 44, 133, 218, 8, 98, 43], OperandSize::Qword)
}

#[test]
fn vmovntpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectScaledDisplaced(EDI, Two, 755814354, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 43, 28, 125, 210, 207, 12, 45], OperandSize::Dword)
}

#[test]
fn vmovntpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Eight, 1578741310, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 253, 40, 43, 164, 209, 62, 174, 25, 94], OperandSize::Qword)
}

#[test]
fn vmovntpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectScaledDisplaced(EBX, Eight, 2103990842, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 253, 72, 43, 44, 221, 58, 90, 104, 125], OperandSize::Dword)
}

#[test]
fn vmovntpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVNTPD, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Two, 1388880925, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 253, 72, 43, 172, 70, 29, 164, 200, 82], OperandSize::Qword)
}

