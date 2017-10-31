use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovlpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 18, 25], OperandSize::Dword)
}

#[test]
fn vmovlpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1014789822, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 18, 28, 245, 190, 118, 124, 60], OperandSize::Qword)
}

#[test]
fn vmovlpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 1476870238, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 18, 20, 133, 94, 64, 7, 88], OperandSize::Dword)
}

#[test]
fn vmovlpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1814399509, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 1, 18, 60, 205, 21, 138, 37, 108], OperandSize::Qword)
}

#[test]
fn vmovlpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Two, 1618573408, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 19, 164, 115, 96, 120, 121, 96], OperandSize::Dword)
}

#[test]
fn vmovlpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(IndirectScaledIndexed(RAX, RBX, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 19, 28, 216], OperandSize::Qword)
}

#[test]
fn vmovlpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Eight, 674533733, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 19, 188, 249, 101, 145, 52, 40], OperandSize::Dword)
}

#[test]
fn vmovlpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVLPD, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Eight, 2090827822, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 253, 8, 19, 140, 243, 46, 128, 159, 124], OperandSize::Qword)
}

