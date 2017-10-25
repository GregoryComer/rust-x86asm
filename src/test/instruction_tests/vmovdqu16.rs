use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmovdqu16_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 255, 137, 111, 206], OperandSize::Dword)
}

fn vmovdqu16_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(XMM4)), operand2: Some(IndirectDisplaced(EBX, 1066393424, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 255, 139, 111, 163, 80, 223, 143, 63], OperandSize::Dword)
}

fn vmovdqu16_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 255, 142, 111, 205], OperandSize::Qword)
}

fn vmovdqu16_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(RCX, RDI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 255, 143, 111, 12, 249], OperandSize::Qword)
}

fn vmovdqu16_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 255, 175, 111, 240], OperandSize::Dword)
}

fn vmovdqu16_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(YMM5)), operand2: Some(IndirectDisplaced(ECX, 840638557, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 255, 173, 111, 169, 93, 32, 27, 50], OperandSize::Dword)
}

fn vmovdqu16_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 145, 255, 172, 111, 254], OperandSize::Qword)
}

fn vmovdqu16_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(YMM20)), operand2: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 255, 170, 111, 32], OperandSize::Qword)
}

fn vmovdqu16_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 255, 207, 111, 249], OperandSize::Dword)
}

fn vmovdqu16_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectDisplaced(ESI, 1358611134, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 255, 202, 111, 134, 190, 194, 250, 80], OperandSize::Dword)
}

fn vmovdqu16_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 255, 201, 111, 252], OperandSize::Qword)
}

fn vmovdqu16_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(ZMM22)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Eight, 1291189500, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 255, 205, 111, 180, 215, 252, 252, 245, 76], OperandSize::Qword)
}

fn vmovdqu16_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 255, 137, 111, 203], OperandSize::Dword)
}

fn vmovdqu16_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(IndirectScaledDisplaced(ECX, Two, 834988525, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 255, 8, 127, 4, 77, 237, 233, 196, 49], OperandSize::Dword)
}

fn vmovdqu16_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 161, 255, 142, 111, 194], OperandSize::Qword)
}

fn vmovdqu16_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(IndirectScaledIndexed(RDX, RDX, Two, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 255, 8, 127, 36, 82], OperandSize::Qword)
}

fn vmovdqu16_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 255, 173, 111, 251], OperandSize::Dword)
}

fn vmovdqu16_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(IndirectDisplaced(EBX, 1775710248, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 255, 40, 127, 171, 40, 48, 215, 105], OperandSize::Dword)
}

fn vmovdqu16_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 145, 255, 171, 111, 196], OperandSize::Qword)
}

fn vmovdqu16_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(IndirectScaledIndexed(RCX, RAX, Two, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 255, 40, 127, 20, 65], OperandSize::Qword)
}

fn vmovdqu16_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 255, 204, 111, 215], OperandSize::Dword)
}

fn vmovdqu16_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Two, 888014973, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 255, 72, 127, 180, 91, 125, 8, 238, 52], OperandSize::Dword)
}

fn vmovdqu16_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 81, 255, 206, 111, 228], OperandSize::Qword)
}

fn vmovdqu16_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQU16, operand1: Some(IndirectDisplaced(RDI, 227637255, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 255, 72, 127, 167, 7, 120, 145, 13], OperandSize::Qword)
}

