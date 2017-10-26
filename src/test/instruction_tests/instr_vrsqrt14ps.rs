use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vrsqrt14ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 78, 193], OperandSize::Dword)
}

#[test]
fn vrsqrt14ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(EDI, ESI, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 140, 78, 4, 119], OperandSize::Dword)
}

#[test]
fn vrsqrt14ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(ESI, 21781016, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 159, 78, 134, 24, 90, 76, 1], OperandSize::Dword)
}

#[test]
fn vrsqrt14ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 2, 125, 141, 78, 237], OperandSize::Qword)
}

#[test]
fn vrsqrt14ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(XMM28)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 125, 141, 78, 32], OperandSize::Qword)
}

#[test]
fn vrsqrt14ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1870534048, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 156, 78, 60, 245, 160, 21, 126, 111], OperandSize::Qword)
}

#[test]
fn vrsqrt14ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 78, 222], OperandSize::Dword)
}

#[test]
fn vrsqrt14ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(YMM1)), operand2: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 78, 14], OperandSize::Dword)
}

#[test]
fn vrsqrt14ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexed(EDI, EBX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 185, 78, 60, 95], OperandSize::Dword)
}

#[test]
fn vrsqrt14ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 194, 125, 169, 78, 197], OperandSize::Qword)
}

#[test]
fn vrsqrt14ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(YMM26)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Two, 512839716, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 125, 175, 78, 148, 73, 36, 80, 145, 30], OperandSize::Qword)
}

#[test]
fn vrsqrt14ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(YMM13)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Two, 1066015489, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 125, 190, 78, 172, 87, 1, 27, 138, 63], OperandSize::Qword)
}

#[test]
fn vrsqrt14ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 78, 227], OperandSize::Dword)
}

#[test]
fn vrsqrt14ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledIndexed(EAX, ECX, Eight, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 202, 78, 36, 200], OperandSize::Dword)
}

#[test]
fn vrsqrt14ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 1342036431, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 220, 78, 36, 117, 207, 217, 253, 79], OperandSize::Dword)
}

#[test]
fn vrsqrt14ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 34, 125, 201, 78, 245], OperandSize::Qword)
}

#[test]
fn vrsqrt14ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(ZMM24)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Eight, 1204011898, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 125, 205, 78, 132, 240, 122, 195, 195, 71], OperandSize::Qword)
}

#[test]
fn vrsqrt14ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VRSQRT14PS, operand1: Some(Direct(ZMM29)), operand2: Some(IndirectDisplaced(RDX, 287537048, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 125, 222, 78, 170, 152, 119, 35, 17], OperandSize::Qword)
}

