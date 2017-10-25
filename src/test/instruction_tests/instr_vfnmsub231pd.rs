use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmsub231pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 190, 237], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Four, 1683023599, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 190, 180, 142, 239, 230, 80, 100], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 190, 230], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Two, 2012665444, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 190, 172, 73, 100, 214, 246, 119], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 190, 228], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 190, 54], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 190, 249], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RDI, RBX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 190, 44, 159], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 213, 137, 190, 206], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 2110791959, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 221, 142, 190, 140, 86, 23, 33, 208, 125], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 221, 159, 190, 60, 65], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 146, 253, 134, 190, 216], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM18)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 237, 131, 190, 47], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 1013912595, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 181, 147, 190, 60, 245, 19, 20, 111, 60], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 172, 190, 203], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Two, 442800714, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 172, 190, 188, 121, 74, 154, 100, 26], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(EDX, 1077210669, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 229, 190, 190, 138, 45, 238, 52, 64], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 2, 253, 169, 190, 226], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM22)), operand3: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 205, 164, 190, 0], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(RDI, 371064617, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 213, 190, 190, 135, 41, 255, 29, 22], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 154, 190, 221], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Two, 1984839235, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 245, 205, 190, 148, 114, 67, 62, 78, 118], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 2023436477, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 221, 190, 36, 77, 189, 48, 155, 120], OperandSize::Dword)
}

#[test]
fn vfnmsub231pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 253, 241, 190, 253], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 133, 194, 190, 20, 115], OperandSize::Qword)
}

#[test]
fn vfnmsub231pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMSUB231PD, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM31)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 133, 209, 190, 40], OperandSize::Qword)
}

