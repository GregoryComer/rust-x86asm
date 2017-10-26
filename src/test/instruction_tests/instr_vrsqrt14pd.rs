use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrt14pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 141, 78, 197], OperandSize::Dword)
}

#[test]
fn vrsqrt14pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(XMM6)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 140, 78, 50], OperandSize::Dword)
}

#[test]
fn vrsqrt14pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 1551836867, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 154, 78, 28, 77, 195, 38, 127, 92], OperandSize::Dword)
}

#[test]
fn vrsqrt14pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 82, 253, 143, 78, 231], OperandSize::Qword)
}

#[test]
fn vrsqrt14pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(XMM23)), operand2: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 253, 137, 78, 62], OperandSize::Qword)
}

#[test]
fn vrsqrt14pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(XMM9)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 1873680089, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 253, 154, 78, 12, 221, 217, 22, 174, 111], OperandSize::Qword)
}

#[test]
fn vrsqrt14pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 171, 78, 221], OperandSize::Dword)
}

#[test]
fn vrsqrt14pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 169, 78, 30], OperandSize::Dword)
}

#[test]
fn vrsqrt14pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectDisplaced(EDX, 8743648, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 185, 78, 138, 224, 106, 133, 0], OperandSize::Dword)
}

#[test]
fn vrsqrt14pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(YMM8)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 253, 169, 78, 198], OperandSize::Qword)
}

#[test]
fn vrsqrt14pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(YMM31)), operand2: Some(IndirectScaledIndexed(RAX, RDI, Two, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 253, 172, 78, 60, 120], OperandSize::Qword)
}

#[test]
fn vrsqrt14pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(YMM10)), operand2: Some(IndirectScaledDisplaced(RAX, Four, 1082950043, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 253, 186, 78, 20, 133, 155, 129, 140, 64], OperandSize::Qword)
}

#[test]
fn vrsqrt14pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 203, 78, 208], OperandSize::Dword)
}

#[test]
fn vrsqrt14pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 562556014, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 201, 78, 4, 85, 110, 236, 135, 33], OperandSize::Dword)
}

#[test]
fn vrsqrt14pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Four, 256492405, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 217, 78, 156, 134, 117, 195, 73, 15], OperandSize::Dword)
}

#[test]
fn vrsqrt14pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 253, 202, 78, 203], OperandSize::Qword)
}

#[test]
fn vrsqrt14pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(ZMM17)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 191353317, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 253, 205, 78, 12, 245, 229, 209, 103, 11], OperandSize::Qword)
}

#[test]
fn vrsqrt14pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PD, operand1: Some(Direct(ZMM17)), operand2: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 253, 221, 78, 15], OperandSize::Qword)
}

