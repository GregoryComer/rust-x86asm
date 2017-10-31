use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovdqu64_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 254, 140, 111, 249], OperandSize::Dword)
}

#[test]
fn vmovdqu64_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EAX, EDX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 254, 140, 111, 44, 80], OperandSize::Dword)
}

#[test]
fn vmovdqu64_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 17, 254, 143, 111, 255], OperandSize::Qword)
}

#[test]
fn vmovdqu64_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(XMM19)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Eight, 86482452, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 254, 137, 111, 156, 250, 20, 158, 39, 5], OperandSize::Qword)
}

#[test]
fn vmovdqu64_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 254, 173, 111, 233], OperandSize::Dword)
}

#[test]
fn vmovdqu64_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(YMM7)), operand2: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 254, 169, 111, 56], OperandSize::Dword)
}

#[test]
fn vmovdqu64_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 254, 172, 111, 228], OperandSize::Qword)
}

#[test]
fn vmovdqu64_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(YMM1)), operand2: Some(IndirectDisplaced(RAX, 2007008936, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 254, 175, 111, 136, 168, 134, 160, 119], OperandSize::Qword)
}

#[test]
fn vmovdqu64_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 254, 205, 111, 228], OperandSize::Dword)
}

#[test]
fn vmovdqu64_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(ZMM7)), operand2: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 254, 201, 111, 58], OperandSize::Dword)
}

#[test]
fn vmovdqu64_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 254, 204, 111, 220], OperandSize::Qword)
}

#[test]
fn vmovdqu64_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(ZMM12)), operand2: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 254, 207, 111, 39], OperandSize::Qword)
}

#[test]
fn vmovdqu64_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 254, 140, 111, 224], OperandSize::Dword)
}

#[test]
fn vmovdqu64_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 254, 8, 127, 30], OperandSize::Dword)
}

#[test]
fn vmovdqu64_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 17, 254, 141, 111, 244], OperandSize::Qword)
}

#[test]
fn vmovdqu64_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 254, 8, 127, 30], OperandSize::Qword)
}

#[test]
fn vmovdqu64_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 254, 173, 111, 230], OperandSize::Dword)
}

#[test]
fn vmovdqu64_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(IndirectDisplaced(EDX, 1414777104, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 254, 40, 127, 154, 16, 201, 83, 84], OperandSize::Dword)
}

#[test]
fn vmovdqu64_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 254, 174, 111, 218], OperandSize::Qword)
}

#[test]
fn vmovdqu64_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 254, 40, 127, 3], OperandSize::Qword)
}

#[test]
fn vmovdqu64_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 254, 204, 111, 226], OperandSize::Dword)
}

#[test]
fn vmovdqu64_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(IndirectScaledDisplaced(EAX, Two, 1355826966, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 254, 72, 127, 28, 69, 22, 71, 208, 80], OperandSize::Dword)
}

#[test]
fn vmovdqu64_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 177, 254, 203, 111, 214], OperandSize::Qword)
}

#[test]
fn vmovdqu64_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU64, operand1: Some(IndirectScaledDisplaced(RCX, Four, 1912454737, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 254, 72, 127, 44, 141, 81, 190, 253, 113], OperandSize::Qword)
}

