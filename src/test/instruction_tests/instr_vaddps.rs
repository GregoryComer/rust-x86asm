use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vaddps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 88, 195], OperandSize::Dword)
}

#[test]
fn vaddps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Four, 1686519211, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 88, 188, 143, 171, 61, 134, 100], OperandSize::Dword)
}

#[test]
fn vaddps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 88, 241], OperandSize::Qword)
}

#[test]
fn vaddps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(RBX, RCX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 88, 36, 139], OperandSize::Qword)
}

#[test]
fn vaddps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 88, 246], OperandSize::Dword)
}

#[test]
fn vaddps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Four, 759210296, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 228, 88, 156, 144, 56, 161, 64, 45], OperandSize::Dword)
}

#[test]
fn vaddps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 88, 235], OperandSize::Qword)
}

#[test]
fn vaddps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Four, 671036084, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 88, 148, 146, 180, 50, 255, 39], OperandSize::Qword)
}

#[test]
fn vaddps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 142, 88, 247], OperandSize::Dword)
}

#[test]
fn vaddps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 1306155504, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 108, 142, 88, 20, 125, 240, 89, 218, 77], OperandSize::Dword)
}

#[test]
fn vaddps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 766294582, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 100, 155, 88, 36, 157, 54, 186, 172, 45], OperandSize::Dword)
}

#[test]
fn vaddps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM26)), operand3: Some(Direct(XMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 193, 44, 130, 88, 200], OperandSize::Qword)
}

#[test]
fn vaddps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Two, 1586327989, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 68, 132, 88, 188, 86, 181, 113, 141, 94], OperandSize::Qword)
}

#[test]
fn vaddps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Eight, 100786365, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 68, 158, 88, 156, 240, 189, 224, 1, 6], OperandSize::Qword)
}

#[test]
fn vaddps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 100, 175, 88, 212], OperandSize::Dword)
}

#[test]
fn vaddps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 1045197340, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 116, 172, 88, 4, 125, 28, 114, 76, 62], OperandSize::Dword)
}

#[test]
fn vaddps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 84, 186, 88, 33], OperandSize::Dword)
}

#[test]
fn vaddps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(YMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 209, 36, 164, 88, 230], OperandSize::Qword)
}

#[test]
fn vaddps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RCX, 1938837797, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 124, 169, 88, 169, 37, 81, 144, 115], OperandSize::Qword)
}

#[test]
fn vaddps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM27)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 36, 178, 88, 59], OperandSize::Qword)
}

#[test]
fn vaddps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 108, 250, 88, 238], OperandSize::Dword)
}

#[test]
fn vaddps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectDisplaced(EBX, 1419192307, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 76, 205, 88, 139, 243, 39, 151, 84], OperandSize::Dword)
}

#[test]
fn vaddps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 100, 222, 88, 11], OperandSize::Dword)
}

#[test]
fn vaddps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM30)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 12, 177, 88, 209], OperandSize::Qword)
}

#[test]
fn vaddps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM18)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 2110990210, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 108, 195, 88, 60, 253, 130, 39, 211, 125], OperandSize::Qword)
}

#[test]
fn vaddps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VADDPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM14)), operand3: Some(IndirectDisplaced(RDI, 1938762868, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 12, 218, 88, 167, 116, 44, 143, 115], OperandSize::Qword)
}

