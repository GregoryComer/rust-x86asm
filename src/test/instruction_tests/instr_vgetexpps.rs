use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vgetexpps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 66, 229], OperandSize::Dword)
}

#[test]
fn vgetexpps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 66, 23], OperandSize::Dword)
}

#[test]
fn vgetexpps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexed(EDI, EAX, Two, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 153, 66, 44, 71], OperandSize::Dword)
}

#[test]
fn vgetexpps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 162, 125, 143, 66, 207], OperandSize::Qword)
}

#[test]
fn vgetexpps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(XMM21)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 1593090056, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 125, 141, 66, 44, 117, 8, 160, 244, 94], OperandSize::Qword)
}

#[test]
fn vgetexpps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(XMM25)), operand2: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 125, 154, 66, 15], OperandSize::Qword)
}

#[test]
fn vgetexpps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 66, 199], OperandSize::Dword)
}

#[test]
fn vgetexpps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Eight, 587328720, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 66, 132, 222, 208, 236, 1, 35], OperandSize::Dword)
}

#[test]
fn vgetexpps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexed(EDX, EDI, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 185, 66, 36, 186], OperandSize::Dword)
}

#[test]
fn vgetexpps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 146, 125, 172, 66, 192], OperandSize::Qword)
}

#[test]
fn vgetexpps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(YMM20)), operand2: Some(IndirectDisplaced(RAX, 413951153, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 125, 171, 66, 160, 177, 100, 172, 24], OperandSize::Qword)
}

#[test]
fn vgetexpps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexed(RDI, RDI, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 188, 66, 12, 255], OperandSize::Qword)
}

#[test]
fn vgetexpps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 155, 66, 235], OperandSize::Dword)
}

#[test]
fn vgetexpps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexed(EAX, EDI, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 204, 66, 28, 184], OperandSize::Dword)
}

#[test]
fn vgetexpps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Two, 477170779, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 223, 66, 132, 122, 91, 12, 113, 28], OperandSize::Dword)
}

#[test]
fn vgetexpps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K6), broadcast: None }, &[98, 34, 125, 158, 66, 207], OperandSize::Qword)
}

#[test]
fn vgetexpps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(ZMM28)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 1847945872, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 125, 204, 66, 36, 213, 144, 106, 37, 110], OperandSize::Qword)
}

#[test]
fn vgetexpps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(ZMM17)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 1320150437, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 125, 221, 66, 12, 181, 165, 229, 175, 78], OperandSize::Qword)
}

