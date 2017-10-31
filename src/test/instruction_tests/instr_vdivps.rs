use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vdivps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 224, 94, 204], OperandSize::Dword)
}

#[test]
fn vdivps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EAX, 604318567, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 94, 128, 103, 43, 5, 36], OperandSize::Dword)
}

#[test]
fn vdivps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 200, 94, 204], OperandSize::Qword)
}

#[test]
fn vdivps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RCX, RCX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 208, 94, 20, 137], OperandSize::Qword)
}

#[test]
fn vdivps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 94, 192], OperandSize::Dword)
}

#[test]
fn vdivps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(EBX, ECX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 94, 52, 203], OperandSize::Dword)
}

#[test]
fn vdivps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 94, 231], OperandSize::Qword)
}

#[test]
fn vdivps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 212, 94, 63], OperandSize::Qword)
}

#[test]
fn vdivps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 76, 141, 94, 210], OperandSize::Dword)
}

#[test]
fn vdivps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EBX, 1756227458, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 84, 142, 94, 171, 130, 231, 173, 104], OperandSize::Dword)
}

#[test]
fn vdivps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Eight, 818935182, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 92, 153, 94, 180, 208, 142, 245, 207, 48], OperandSize::Dword)
}

#[test]
fn vdivps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 92, 135, 94, 250], OperandSize::Qword)
}

#[test]
fn vdivps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1622464218, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 76, 131, 94, 44, 197, 218, 214, 180, 96], OperandSize::Qword)
}

#[test]
fn vdivps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RAX, Eight, 1312777262, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 113, 84, 146, 94, 156, 194, 46, 100, 63, 78], OperandSize::Qword)
}

#[test]
fn vdivps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 68, 173, 94, 243], OperandSize::Dword)
}

#[test]
fn vdivps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EDX, 856001208, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 68, 174, 94, 138, 184, 138, 5, 51], OperandSize::Dword)
}

#[test]
fn vdivps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Two, 702036468, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 100, 186, 94, 132, 81, 244, 57, 216, 41], OperandSize::Dword)
}

#[test]
fn vdivps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM17)), operand3: Some(Direct(YMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 161, 116, 166, 94, 199], OperandSize::Qword)
}

#[test]
fn vdivps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectDisplaced(RAX, 1741774482, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 100, 164, 94, 128, 146, 94, 209, 103], OperandSize::Qword)
}

#[test]
fn vdivps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 116, 181, 94, 4, 240], OperandSize::Qword)
}

#[test]
fn vdivps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 116, 186, 94, 201], OperandSize::Dword)
}

#[test]
fn vdivps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 84, 204, 94, 19], OperandSize::Dword)
}

#[test]
fn vdivps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 76, 219, 94, 16], OperandSize::Dword)
}

#[test]
fn vdivps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM22)), operand3: Some(Direct(ZMM24)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 17, 76, 180, 94, 232], OperandSize::Qword)
}

#[test]
fn vdivps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Four, 619207408, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 92, 206, 94, 156, 159, 240, 90, 232, 36], OperandSize::Qword)
}

#[test]
fn vdivps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPS, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM24)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 113, 60, 215, 94, 58], OperandSize::Qword)
}

