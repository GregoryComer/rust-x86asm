use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovdqu8_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 127, 137, 111, 251], OperandSize::Dword)
}

#[test]
fn vmovdqu8_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 127, 139, 111, 58], OperandSize::Dword)
}

#[test]
fn vmovdqu8_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 177, 127, 140, 111, 242], OperandSize::Qword)
}

#[test]
fn vmovdqu8_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(XMM24)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 775597279, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 127, 141, 111, 4, 133, 223, 172, 58, 46], OperandSize::Qword)
}

#[test]
fn vmovdqu8_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 127, 171, 111, 204], OperandSize::Dword)
}

#[test]
fn vmovdqu8_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 127, 169, 111, 46], OperandSize::Dword)
}

#[test]
fn vmovdqu8_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 1, 127, 174, 111, 198], OperandSize::Qword)
}

#[test]
fn vmovdqu8_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Four, 1259352289, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 127, 169, 111, 164, 129, 225, 48, 16, 75], OperandSize::Qword)
}

#[test]
fn vmovdqu8_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 127, 204, 111, 209], OperandSize::Dword)
}

#[test]
fn vmovdqu8_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexed(ESI, ESI, Eight, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 127, 206, 111, 20, 246], OperandSize::Dword)
}

#[test]
fn vmovdqu8_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 127, 205, 111, 224], OperandSize::Qword)
}

#[test]
fn vmovdqu8_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Four, 593066210, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 127, 206, 111, 132, 158, 226, 120, 89, 35], OperandSize::Qword)
}

#[test]
fn vmovdqu8_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 127, 139, 111, 198], OperandSize::Dword)
}

#[test]
fn vmovdqu8_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(IndirectScaledIndexed(EBX, EAX, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 8, 127, 44, 131], OperandSize::Dword)
}

#[test]
fn vmovdqu8_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 17, 127, 143, 111, 222], OperandSize::Qword)
}

#[test]
fn vmovdqu8_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(IndirectScaledIndexed(RCX, RSI, Two, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 127, 8, 127, 52, 113], OperandSize::Qword)
}

#[test]
fn vmovdqu8_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 127, 170, 111, 209], OperandSize::Dword)
}

#[test]
fn vmovdqu8_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Two, 443660825, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 40, 127, 140, 114, 25, 186, 113, 26], OperandSize::Dword)
}

#[test]
fn vmovdqu8_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 127, 171, 111, 229], OperandSize::Qword)
}

#[test]
fn vmovdqu8_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(IndirectDisplaced(RAX, 1940660910, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 127, 40, 127, 160, 174, 34, 172, 115], OperandSize::Qword)
}

#[test]
fn vmovdqu8_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 127, 201, 111, 246], OperandSize::Dword)
}

#[test]
fn vmovdqu8_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(IndirectDisplaced(ECX, 1401445775, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 127, 72, 127, 161, 143, 93, 136, 83], OperandSize::Dword)
}

#[test]
fn vmovdqu8_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 161, 127, 207, 111, 254], OperandSize::Qword)
}

#[test]
fn vmovdqu8_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU8, operand1: Some(IndirectScaledDisplaced(RSI, Four, 305664799, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 127, 72, 127, 28, 181, 31, 19, 56, 18], OperandSize::Qword)
}

