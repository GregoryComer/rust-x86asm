use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsrlvq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 69, 238], OperandSize::Dword)
}

#[test]
fn vpsrlvq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 69, 39], OperandSize::Dword)
}

#[test]
fn vpsrlvq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 69, 225], OperandSize::Qword)
}

#[test]
fn vpsrlvq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 249, 69, 39], OperandSize::Qword)
}

#[test]
fn vpsrlvq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 69, 229], OperandSize::Dword)
}

#[test]
fn vpsrlvq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(ECX, EDX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 69, 28, 81], OperandSize::Dword)
}

#[test]
fn vpsrlvq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 69, 217], OperandSize::Qword)
}

#[test]
fn vpsrlvq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(RAX, 1321270054, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 69, 136, 38, 251, 192, 78], OperandSize::Qword)
}

#[test]
fn vpsrlvq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 221, 141, 69, 207], OperandSize::Dword)
}

#[test]
fn vpsrlvq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(ESI, EBX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 140, 69, 36, 94], OperandSize::Dword)
}

#[test]
fn vpsrlvq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 205, 157, 69, 52, 192], OperandSize::Dword)
}

#[test]
fn vpsrlvq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM27)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 130, 165, 135, 69, 229], OperandSize::Qword)
}

#[test]
fn vpsrlvq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Two, 799297330, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 157, 139, 69, 156, 113, 50, 79, 164, 47], OperandSize::Qword)
}

#[test]
fn vpsrlvq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1964858302, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 253, 148, 69, 60, 157, 190, 91, 29, 117], OperandSize::Qword)
}

#[test]
fn vpsrlvq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 229, 172, 69, 231], OperandSize::Dword)
}

#[test]
fn vpsrlvq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(ECX, ESI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 221, 175, 69, 36, 113], OperandSize::Dword)
}

#[test]
fn vpsrlvq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EDX, 2048090531, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 188, 69, 178, 163, 97, 19, 122], OperandSize::Dword)
}

#[test]
fn vpsrlvq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM24)), operand3: Some(Direct(YMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 82, 189, 164, 69, 246], OperandSize::Qword)
}

#[test]
fn vpsrlvq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 189, 175, 69, 4, 80], OperandSize::Qword)
}

#[test]
fn vpsrlvq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(YMM27)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Two, 1384440032, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 189, 186, 69, 156, 94, 224, 224, 132, 82], OperandSize::Qword)
}

#[test]
fn vpsrlvq_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 245, 205, 69, 220], OperandSize::Dword)
}

#[test]
fn vpsrlvq_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(EAX, 1150616832, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 213, 203, 69, 160, 0, 5, 149, 68], OperandSize::Dword)
}

#[test]
fn vpsrlvq_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Four, 2126338351, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 222, 69, 132, 142, 47, 89, 189, 126], OperandSize::Dword)
}

#[test]
fn vpsrlvq_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM18)), operand3: Some(Direct(ZMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 18, 237, 198, 69, 250], OperandSize::Qword)
}

#[test]
fn vpsrlvq_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1052040671, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 213, 196, 69, 36, 189, 223, 221, 180, 62], OperandSize::Qword)
}

#[test]
fn vpsrlvq_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVQ, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 213, 220, 69, 14], OperandSize::Qword)
}

