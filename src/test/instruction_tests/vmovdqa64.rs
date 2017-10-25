use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmovdqa64_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 139, 111, 206], OperandSize::Dword)
}

fn vmovdqa64_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(XMM6)), operand2: Some(IndirectScaledIndexed(EDX, EDI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 137, 111, 52, 186], OperandSize::Dword)
}

fn vmovdqa64_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 193, 253, 138, 111, 203], OperandSize::Qword)
}

fn vmovdqa64_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(XMM23)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Four, 1662246851, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 253, 138, 111, 188, 153, 195, 223, 19, 99], OperandSize::Qword)
}

fn vmovdqa64_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 253, 172, 111, 210], OperandSize::Dword)
}

fn vmovdqa64_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(YMM1)), operand2: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 253, 170, 111, 14], OperandSize::Dword)
}

fn vmovdqa64_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 175, 111, 234], OperandSize::Qword)
}

fn vmovdqa64_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(YMM2)), operand2: Some(IndirectDisplaced(RBX, 1488327641, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 175, 111, 147, 217, 19, 182, 88], OperandSize::Qword)
}

fn vmovdqa64_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 201, 111, 200], OperandSize::Dword)
}

fn vmovdqa64_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexed(EBX, EDI, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 203, 111, 36, 123], OperandSize::Dword)
}

fn vmovdqa64_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 253, 207, 111, 196], OperandSize::Qword)
}

fn vmovdqa64_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(ZMM19)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 1269035530, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 253, 206, 111, 28, 205, 10, 242, 163, 75], OperandSize::Qword)
}

fn vmovdqa64_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 253, 137, 111, 255], OperandSize::Dword)
}

fn vmovdqa64_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(IndirectDisplaced(EDI, 1793534711, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 253, 8, 127, 159, 247, 42, 231, 106], OperandSize::Dword)
}

fn vmovdqa64_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 81, 253, 142, 111, 212], OperandSize::Qword)
}

fn vmovdqa64_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(IndirectScaledDisplaced(RAX, Eight, 1466392278, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 113, 253, 8, 127, 12, 197, 214, 94, 103, 87], OperandSize::Qword)
}

fn vmovdqa64_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 171, 111, 243], OperandSize::Dword)
}

fn vmovdqa64_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 253, 40, 127, 14], OperandSize::Dword)
}

fn vmovdqa64_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 129, 253, 174, 111, 240], OperandSize::Qword)
}

fn vmovdqa64_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(IndirectScaledDisplaced(RBX, Four, 1002809406, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 225, 253, 40, 127, 4, 157, 62, 168, 197, 59], OperandSize::Qword)
}

fn vmovdqa64_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 253, 203, 111, 201], OperandSize::Dword)
}

fn vmovdqa64_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(IndirectScaledIndexed(EDX, ECX, Four, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 253, 72, 127, 44, 138], OperandSize::Dword)
}

fn vmovdqa64_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 253, 205, 111, 253], OperandSize::Qword)
}

fn vmovdqa64_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVDQA64, operand1: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 253, 72, 127, 54], OperandSize::Qword)
}

