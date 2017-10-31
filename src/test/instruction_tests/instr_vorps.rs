use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vorps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 86, 241], OperandSize::Dword)
}

#[test]
fn vorps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 1954266971, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 216, 86, 12, 85, 91, 191, 123, 116], OperandSize::Dword)
}

#[test]
fn vorps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 86, 217], OperandSize::Qword)
}

#[test]
fn vorps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 375302894, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 86, 52, 85, 238, 170, 94, 22], OperandSize::Qword)
}

#[test]
fn vorps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 244, 86, 255], OperandSize::Dword)
}

#[test]
fn vorps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 236, 86, 50], OperandSize::Dword)
}

#[test]
fn vorps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 86, 234], OperandSize::Qword)
}

#[test]
fn vorps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 86, 44, 193], OperandSize::Qword)
}

#[test]
fn vorps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 92, 141, 86, 248], OperandSize::Dword)
}

#[test]
fn vorps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(EDX, 883436588, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 108, 143, 86, 154, 44, 44, 168, 52], OperandSize::Dword)
}

#[test]
fn vorps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Eight, 1467210657, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 84, 157, 86, 164, 218, 161, 219, 115, 87], OperandSize::Dword)
}

#[test]
fn vorps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM29)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 49, 20, 131, 86, 208], OperandSize::Qword)
}

#[test]
fn vorps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectDisplaced(RDI, 1246700109, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 52, 138, 86, 183, 77, 34, 79, 74], OperandSize::Qword)
}

#[test]
fn vorps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM14)), operand3: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 12, 157, 86, 0], OperandSize::Qword)
}

#[test]
fn vorps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 108, 170, 86, 235], OperandSize::Dword)
}

#[test]
fn vorps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EDI, 2013563448, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 76, 172, 86, 135, 56, 138, 4, 120], OperandSize::Dword)
}

#[test]
fn vorps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 35809962, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 116, 187, 86, 28, 253, 170, 106, 34, 2], OperandSize::Dword)
}

#[test]
fn vorps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM29)), operand3: Some(Direct(YMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 161, 20, 166, 86, 229], OperandSize::Qword)
}

#[test]
fn vorps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM14)), operand3: Some(IndirectScaledIndexed(RAX, RAX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 12, 171, 86, 60, 64], OperandSize::Qword)
}

#[test]
fn vorps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM22)), operand3: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 76, 183, 86, 17], OperandSize::Qword)
}

#[test]
fn vorps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 108, 206, 86, 210], OperandSize::Dword)
}

#[test]
fn vorps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(EBX, 1808948920, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 202, 86, 163, 184, 94, 210, 107], OperandSize::Dword)
}

#[test]
fn vorps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 76, 222, 86, 9], OperandSize::Dword)
}

#[test]
fn vorps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM22)), operand3: Some(Direct(ZMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 81, 76, 197, 86, 212], OperandSize::Qword)
}

#[test]
fn vorps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM23)), operand3: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 68, 199, 86, 9], OperandSize::Qword)
}

#[test]
fn vorps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VORPS, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM26)), operand3: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 113, 44, 209, 86, 2], OperandSize::Qword)
}

