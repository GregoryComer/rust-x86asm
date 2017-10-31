use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermi2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 197, 140, 119, 223], OperandSize::Dword)
}

#[test]
fn vpermi2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 197, 137, 119, 26], OperandSize::Dword)
}

#[test]
fn vpermi2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 50019030, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 197, 155, 119, 60, 85, 214, 58, 251, 2], OperandSize::Dword)
}

#[test]
fn vpermi2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 197, 129, 119, 237], OperandSize::Qword)
}

#[test]
fn vpermi2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectDisplaced(RSI, 32661155, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 133, 142, 119, 142, 163, 94, 242, 1], OperandSize::Qword)
}

#[test]
fn vpermi2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledIndexed(RSI, RDI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 229, 149, 119, 60, 190], OperandSize::Qword)
}

#[test]
fn vpermi2pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 221, 174, 119, 225], OperandSize::Dword)
}

#[test]
fn vpermi2pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 862439050, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 237, 172, 119, 140, 215, 138, 198, 103, 51], OperandSize::Dword)
}

#[test]
fn vpermi2pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EDI, 1194516026, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 190, 119, 167, 58, 222, 50, 71], OperandSize::Dword)
}

#[test]
fn vpermi2pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(YMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 194, 133, 169, 119, 197], OperandSize::Qword)
}

#[test]
fn vpermi2pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM31)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 133, 164, 119, 20, 146], OperandSize::Qword)
}

#[test]
fn vpermi2pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1729480091, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 253, 182, 119, 52, 181, 155, 197, 21, 103], OperandSize::Qword)
}

#[test]
fn vpermi2pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 213, 202, 119, 214], OperandSize::Dword)
}

#[test]
fn vpermi2pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Four, 312649859, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 229, 201, 119, 156, 129, 131, 168, 162, 18], OperandSize::Dword)
}

#[test]
fn vpermi2pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 197, 222, 119, 27], OperandSize::Dword)
}

#[test]
fn vpermi2pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 210, 213, 204, 119, 192], OperandSize::Qword)
}

#[test]
fn vpermi2pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM11)), operand3: Some(IndirectDisplaced(RDI, 876318214, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 165, 203, 119, 183, 6, 142, 59, 52], OperandSize::Qword)
}

#[test]
fn vpermi2pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM8)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 189, 217, 119, 50], OperandSize::Qword)
}

