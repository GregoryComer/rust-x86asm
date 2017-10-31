use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vdivpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 94, 206], OperandSize::Dword)
}

#[test]
fn vdivpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EDX, ESI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 94, 20, 114], OperandSize::Dword)
}

#[test]
fn vdivpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 94, 236], OperandSize::Qword)
}

#[test]
fn vdivpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Four, 1403803889, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 94, 148, 150, 241, 88, 172, 83], OperandSize::Qword)
}

#[test]
fn vdivpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 94, 223], OperandSize::Dword)
}

#[test]
fn vdivpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 856302299, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 94, 44, 77, 219, 34, 10, 51], OperandSize::Dword)
}

#[test]
fn vdivpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 94, 255], OperandSize::Qword)
}

#[test]
fn vdivpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Eight, 1807639542, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 94, 164, 201, 246, 99, 190, 107], OperandSize::Qword)
}

#[test]
fn vdivpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 253, 143, 94, 202], OperandSize::Dword)
}

#[test]
fn vdivpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 643755696, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 213, 137, 94, 132, 86, 176, 238, 94, 38], OperandSize::Dword)
}

#[test]
fn vdivpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 213, 153, 94, 24], OperandSize::Dword)
}

#[test]
fn vdivpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 81, 229, 129, 94, 208], OperandSize::Qword)
}

#[test]
fn vdivpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectDisplaced(RAX, 317710891, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 221, 135, 94, 184, 43, 226, 239, 18], OperandSize::Qword)
}

#[test]
fn vdivpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledIndexed(RDI, RAX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 113, 181, 148, 94, 20, 135], OperandSize::Qword)
}

#[test]
fn vdivpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 221, 170, 94, 225], OperandSize::Dword)
}

#[test]
fn vdivpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(ESI, 1093229976, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 221, 172, 94, 158, 152, 93, 41, 65], OperandSize::Dword)
}

#[test]
fn vdivpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EDI, 1658204191, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 221, 191, 94, 167, 31, 48, 214, 98], OperandSize::Dword)
}

#[test]
fn vdivpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM28)), operand3: Some(Direct(YMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 129, 157, 163, 94, 230], OperandSize::Qword)
}

#[test]
fn vdivpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM31)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 51425001, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 133, 162, 94, 52, 93, 233, 174, 16, 3], OperandSize::Qword)
}

#[test]
fn vdivpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectDisplaced(RDX, 1275017430, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 157, 179, 94, 146, 214, 56, 255, 75], OperandSize::Qword)
}

#[test]
fn vdivpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 237, 223, 94, 249], OperandSize::Dword)
}

#[test]
fn vdivpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 245, 206, 94, 4, 126], OperandSize::Dword)
}

#[test]
fn vdivpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 71111021, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 229, 221, 94, 36, 205, 109, 17, 61, 4], OperandSize::Dword)
}

#[test]
fn vdivpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM19)), operand3: Some(Direct(ZMM14)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 209, 229, 209, 94, 254], OperandSize::Qword)
}

#[test]
fn vdivpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM18)), operand3: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 237, 199, 94, 46], OperandSize::Qword)
}

#[test]
fn vdivpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVPD, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM27)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Eight, 1569684574, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 165, 214, 94, 140, 223, 94, 124, 143, 93], OperandSize::Qword)
}

