use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vsqrtps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 81, 220], OperandSize::Dword)
}

#[test]
fn vsqrtps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 225727456, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 81, 44, 125, 224, 83, 116, 13], OperandSize::Dword)
}

#[test]
fn vsqrtps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 81, 243], OperandSize::Qword)
}

#[test]
fn vsqrtps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Four, 1343205735, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 81, 188, 152, 103, 177, 15, 80], OperandSize::Qword)
}

#[test]
fn vsqrtps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 81, 207], OperandSize::Dword)
}

#[test]
fn vsqrtps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexed(ESI, EDI, Eight, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 81, 44, 254], OperandSize::Dword)
}

#[test]
fn vsqrtps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 81, 244], OperandSize::Qword)
}

#[test]
fn vsqrtps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 81, 19], OperandSize::Qword)
}

#[test]
fn vsqrtps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 139, 81, 236], OperandSize::Dword)
}

#[test]
fn vsqrtps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Two, 1518029468, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 142, 81, 132, 94, 156, 74, 123, 90], OperandSize::Dword)
}

#[test]
fn vsqrtps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Eight, 1967052245, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 124, 154, 81, 140, 240, 213, 213, 62, 117], OperandSize::Dword)
}

#[test]
fn vsqrtps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 124, 139, 81, 199], OperandSize::Qword)
}

#[test]
fn vsqrtps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM9)), operand2: Some(IndirectDisplaced(RDI, 873683348, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 124, 143, 81, 143, 148, 89, 19, 52], OperandSize::Qword)
}

#[test]
fn vsqrtps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(XMM26)), operand2: Some(IndirectDisplaced(RAX, 1594921437, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 124, 159, 81, 144, 221, 145, 16, 95], OperandSize::Qword)
}

#[test]
fn vsqrtps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 169, 81, 244], OperandSize::Dword)
}

#[test]
fn vsqrtps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 1254296627, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 172, 81, 12, 253, 51, 12, 195, 74], OperandSize::Dword)
}

#[test]
fn vsqrtps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 760566320, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 124, 190, 81, 60, 181, 48, 82, 85, 45], OperandSize::Dword)
}

#[test]
fn vsqrtps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 129, 124, 172, 81, 203], OperandSize::Qword)
}

#[test]
fn vsqrtps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM17)), operand2: Some(IndirectDisplaced(RCX, 656308656, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 124, 170, 81, 137, 176, 121, 30, 39], OperandSize::Qword)
}

#[test]
fn vsqrtps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(YMM23)), operand2: Some(IndirectDisplaced(RDX, 721389180, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 225, 124, 188, 81, 186, 124, 134, 255, 42], OperandSize::Qword)
}

#[test]
fn vsqrtps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 249, 81, 250], OperandSize::Dword)
}

#[test]
fn vsqrtps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(ZMM7)), operand2: Some(Indirect(EBX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 202, 81, 59], OperandSize::Dword)
}

#[test]
fn vsqrtps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(ZMM2)), operand2: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 124, 223, 81, 16], OperandSize::Dword)
}

#[test]
fn vsqrtps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(ZMM20)), operand2: Some(Direct(ZMM21)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 161, 124, 219, 81, 229], OperandSize::Qword)
}

#[test]
fn vsqrtps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(ZMM25)), operand2: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 124, 207, 81, 10], OperandSize::Qword)
}

#[test]
fn vsqrtps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTPS, operand1: Some(Direct(ZMM10)), operand2: Some(IndirectDisplaced(RSI, 1411264680, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 113, 124, 223, 81, 150, 168, 48, 30, 84], OperandSize::Qword)
}

