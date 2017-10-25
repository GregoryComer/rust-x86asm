use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vandps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 84, 237], OperandSize::Dword)
}

#[test]
fn vandps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 84, 12, 153], OperandSize::Dword)
}

#[test]
fn vandps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 200, 84, 193], OperandSize::Qword)
}

#[test]
fn vandps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RCX, 1320612927, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 84, 185, 63, 244, 182, 78], OperandSize::Qword)
}

#[test]
fn vandps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 84, 205], OperandSize::Dword)
}

#[test]
fn vandps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 1533665135, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 212, 84, 44, 93, 111, 223, 105, 91], OperandSize::Dword)
}

#[test]
fn vandps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 84, 192], OperandSize::Qword)
}

#[test]
fn vandps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 228, 84, 9], OperandSize::Qword)
}

#[test]
fn vandps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 100, 142, 84, 192], OperandSize::Dword)
}

#[test]
fn vandps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Two, 557634198, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 108, 141, 84, 188, 81, 150, 210, 60, 33], OperandSize::Dword)
}

#[test]
fn vandps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EAX, EDI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 124, 154, 84, 28, 120], OperandSize::Dword)
}

#[test]
fn vandps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 145, 28, 132, 84, 245], OperandSize::Qword)
}

#[test]
fn vandps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM25)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 52, 134, 84, 11], OperandSize::Qword)
}

#[test]
fn vandps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledIndexed(RBX, RDI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 44, 154, 84, 28, 187], OperandSize::Qword)
}

#[test]
fn vandps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 100, 173, 84, 199], OperandSize::Dword)
}

#[test]
fn vandps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 68, 171, 84, 20, 71], OperandSize::Dword)
}

#[test]
fn vandps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 92, 186, 84, 52, 115], OperandSize::Dword)
}

#[test]
fn vandps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM22)), operand3: Some(Direct(YMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 145, 76, 166, 84, 230], OperandSize::Qword)
}

#[test]
fn vandps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM14)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 12, 175, 84, 9], OperandSize::Qword)
}

#[test]
fn vandps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Two, 1765815808, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 92, 182, 84, 172, 79, 0, 54, 64, 105], OperandSize::Qword)
}

#[test]
fn vandps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 84, 205, 84, 217], OperandSize::Dword)
}

#[test]
fn vandps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 207, 84, 14], OperandSize::Dword)
}

#[test]
fn vandps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 1400109547, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 84, 217, 84, 4, 245, 235, 249, 115, 83], OperandSize::Dword)
}

#[test]
fn vandps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM19)), operand3: Some(Direct(ZMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 209, 100, 196, 84, 245], OperandSize::Qword)
}

#[test]
fn vandps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Two, 1191270444, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 108, 203, 84, 180, 87, 44, 88, 1, 71], OperandSize::Qword)
}

#[test]
fn vandps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VANDPS, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM12)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 225, 28, 222, 84, 12, 184], OperandSize::Qword)
}

