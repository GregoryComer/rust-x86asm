use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovddup_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 18, 196], OperandSize::Dword)
}

#[test]
fn vmovddup_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Four, 490443215, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 18, 164, 135, 207, 145, 59, 29], OperandSize::Dword)
}

#[test]
fn vmovddup_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 18, 223], OperandSize::Qword)
}

#[test]
fn vmovddup_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RBX, RSI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 18, 36, 115], OperandSize::Qword)
}

#[test]
fn vmovddup_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 18, 213], OperandSize::Dword)
}

#[test]
fn vmovddup_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Two, 1569892213, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 18, 148, 80, 117, 167, 146, 93], OperandSize::Dword)
}

#[test]
fn vmovddup_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 18, 206], OperandSize::Qword)
}

#[test]
fn vmovddup_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 1885671548, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 18, 20, 221, 124, 16, 101, 112], OperandSize::Qword)
}

#[test]
fn vmovddup_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 255, 139, 18, 196], OperandSize::Dword)
}

#[test]
fn vmovddup_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 1587784959, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 255, 137, 18, 60, 85, 255, 172, 163, 94], OperandSize::Dword)
}

#[test]
fn vmovddup_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 65, 255, 142, 18, 227], OperandSize::Qword)
}

#[test]
fn vmovddup_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(XMM22)), operand2: Some(IndirectDisplaced(RDI, 948591254, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 255, 138, 18, 183, 150, 90, 138, 56], OperandSize::Qword)
}

#[test]
fn vmovddup_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 255, 171, 18, 226], OperandSize::Dword)
}

#[test]
fn vmovddup_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Two, 795835010, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 255, 174, 18, 164, 71, 130, 122, 111, 47], OperandSize::Dword)
}

#[test]
fn vmovddup_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 193, 255, 170, 18, 204], OperandSize::Qword)
}

#[test]
fn vmovddup_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(YMM15)), operand2: Some(IndirectDisplaced(RDX, 771856498, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 255, 174, 18, 186, 114, 152, 1, 46], OperandSize::Qword)
}

#[test]
fn vmovddup_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 255, 201, 18, 229], OperandSize::Dword)
}

#[test]
fn vmovddup_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Four, 1921211732, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 255, 202, 18, 148, 151, 84, 93, 131, 114], OperandSize::Dword)
}

#[test]
fn vmovddup_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 33, 255, 204, 18, 200], OperandSize::Qword)
}

#[test]
fn vmovddup_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDDUP, operand1: Some(Direct(ZMM15)), operand2: Some(IndirectScaledIndexed(RDI, RAX, Eight, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 255, 201, 18, 60, 199], OperandSize::Qword)
}

