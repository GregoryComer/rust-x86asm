use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vunpckhps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 21, 247], OperandSize::Dword)
}

#[test]
fn vunpckhps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Eight, 2079449201, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 21, 180, 249, 113, 224, 241, 123], OperandSize::Dword)
}

#[test]
fn vunpckhps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 21, 220], OperandSize::Qword)
}

#[test]
fn vunpckhps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 240, 21, 44, 202], OperandSize::Qword)
}

#[test]
fn vunpckhps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 21, 252], OperandSize::Dword)
}

#[test]
fn vunpckhps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 212, 21, 51], OperandSize::Dword)
}

#[test]
fn vunpckhps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 21, 255], OperandSize::Qword)
}

#[test]
fn vunpckhps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 370964100, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 21, 4, 181, 132, 118, 28, 22], OperandSize::Qword)
}

#[test]
fn vunpckhps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 84, 139, 21, 209], OperandSize::Dword)
}

#[test]
fn vunpckhps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 108, 138, 21, 20, 194], OperandSize::Dword)
}

#[test]
fn vunpckhps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Eight, 646038773, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 84, 159, 21, 156, 211, 245, 196, 129, 38], OperandSize::Dword)
}

#[test]
fn vunpckhps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 49, 76, 142, 21, 193], OperandSize::Qword)
}

#[test]
fn vunpckhps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Eight, 1019819038, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 20, 138, 21, 188, 201, 30, 52, 201, 60], OperandSize::Qword)
}

#[test]
fn vunpckhps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 76, 155, 21, 20, 83], OperandSize::Qword)
}

#[test]
fn vunpckhps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 92, 170, 21, 202], OperandSize::Dword)
}

#[test]
fn vunpckhps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 100, 173, 21, 27], OperandSize::Dword)
}

#[test]
fn vunpckhps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1551659050, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 92, 191, 21, 20, 253, 42, 112, 124, 92], OperandSize::Dword)
}

#[test]
fn vunpckhps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 145, 116, 174, 21, 213], OperandSize::Qword)
}

#[test]
fn vunpckhps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM22)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 76, 164, 21, 2], OperandSize::Qword)
}

#[test]
fn vunpckhps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1772664256, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 97, 60, 178, 21, 44, 157, 192, 181, 168, 105], OperandSize::Qword)
}

#[test]
fn vunpckhps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 108, 204, 21, 245], OperandSize::Dword)
}

#[test]
fn vunpckhps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectDisplaced(ESI, 2047035106, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 68, 207, 21, 190, 226, 70, 3, 122], OperandSize::Dword)
}

#[test]
fn vunpckhps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(EDX, 1942912783, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 92, 223, 21, 146, 15, 127, 206, 115], OperandSize::Dword)
}

#[test]
fn vunpckhps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 49, 60, 193, 21, 210], OperandSize::Qword)
}

#[test]
fn vunpckhps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectDisplaced(RAX, 1365837855, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 76, 193, 21, 160, 31, 8, 105, 81], OperandSize::Qword)
}

#[test]
fn vunpckhps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VUNPCKHPS, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM18)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Two, 1207952973, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 97, 108, 214, 21, 188, 86, 77, 230, 255, 71], OperandSize::Qword)
}

