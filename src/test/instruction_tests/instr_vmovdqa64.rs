use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmovdqa64_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 139, 111, 251], OperandSize::Dword)
}

#[test]
fn vmovdqa64_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(EDI, EBX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 142, 111, 20, 223], OperandSize::Dword)
}

#[test]
fn vmovdqa64_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 177, 253, 142, 111, 215], OperandSize::Qword)
}

#[test]
fn vmovdqa64_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(XMM21)), operand2: Some(IndirectDisplaced(RBX, 122436272, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 253, 143, 111, 171, 176, 58, 76, 7], OperandSize::Qword)
}

#[test]
fn vmovdqa64_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 174, 111, 231], OperandSize::Dword)
}

#[test]
fn vmovdqa64_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(YMM5)), operand2: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 171, 111, 40], OperandSize::Dword)
}

#[test]
fn vmovdqa64_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 65, 253, 175, 111, 227], OperandSize::Qword)
}

#[test]
fn vmovdqa64_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(YMM12)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Four, 1809334581, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 253, 169, 111, 164, 159, 53, 65, 216, 107], OperandSize::Qword)
}

#[test]
fn vmovdqa64_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 204, 111, 224], OperandSize::Dword)
}

#[test]
fn vmovdqa64_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectDisplaced(EDI, 761628869, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 204, 111, 167, 197, 136, 101, 45], OperandSize::Dword)
}

#[test]
fn vmovdqa64_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 65, 253, 207, 111, 236], OperandSize::Qword)
}

#[test]
fn vmovdqa64_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Four, 360649149, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 201, 111, 180, 182, 189, 17, 127, 21], OperandSize::Qword)
}

#[test]
fn vmovdqa64_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 137, 111, 243], OperandSize::Dword)
}

#[test]
fn vmovdqa64_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(IndirectDisplaced(ESI, 1415020857, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 253, 8, 127, 174, 57, 129, 87, 84], OperandSize::Dword)
}

#[test]
fn vmovdqa64_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 1, 253, 140, 111, 252], OperandSize::Qword)
}

#[test]
fn vmovdqa64_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(IndirectScaledIndexed(RDI, RDX, Two, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 253, 8, 127, 60, 87], OperandSize::Qword)
}

#[test]
fn vmovdqa64_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 174, 111, 226], OperandSize::Dword)
}

#[test]
fn vmovdqa64_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(IndirectDisplaced(EDI, 537258592, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 253, 40, 127, 151, 96, 234, 5, 32], OperandSize::Dword)
}

#[test]
fn vmovdqa64_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 177, 253, 175, 111, 246], OperandSize::Qword)
}

#[test]
fn vmovdqa64_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RBX, Eight, 773870247, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 253, 40, 127, 172, 219, 167, 82, 32, 46], OperandSize::Qword)
}

#[test]
fn vmovdqa64_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 253, 206, 111, 208], OperandSize::Dword)
}

#[test]
fn vmovdqa64_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(IndirectDisplaced(ECX, 1377197729, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 253, 72, 127, 129, 161, 94, 22, 82], OperandSize::Dword)
}

#[test]
fn vmovdqa64_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 17, 253, 207, 111, 254], OperandSize::Qword)
}

#[test]
fn vmovdqa64_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(IndirectScaledIndexed(RCX, RDX, Eight, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 97, 253, 72, 127, 4, 209], OperandSize::Qword)
}

