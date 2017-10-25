use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vgetexpps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 141, 66, 205], OperandSize::Dword)
}

#[test]
fn vgetexpps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Two, 1685323213, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 66, 156, 90, 205, 253, 115, 100], OperandSize::Dword)
}

#[test]
fn vgetexpps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 1694548894, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 158, 66, 4, 141, 158, 195, 0, 101], OperandSize::Dword)
}

#[test]
fn vgetexpps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 82, 125, 142, 66, 239], OperandSize::Qword)
}

#[test]
fn vgetexpps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(XMM17)), operand2: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 125, 142, 66, 15], OperandSize::Qword)
}

#[test]
fn vgetexpps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(XMM2)), operand2: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 157, 66, 19], OperandSize::Qword)
}

#[test]
fn vgetexpps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 66, 250], OperandSize::Dword)
}

#[test]
fn vgetexpps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledDisplaced(ECX, Eight, 987308230, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 175, 66, 36, 205, 198, 32, 217, 58], OperandSize::Dword)
}

#[test]
fn vgetexpps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(EBX, EAX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 185, 66, 20, 195], OperandSize::Dword)
}

#[test]
fn vgetexpps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 210, 125, 172, 66, 234], OperandSize::Qword)
}

#[test]
fn vgetexpps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(YMM29)), operand2: Some(IndirectDisplaced(RBX, 428588566, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 125, 169, 66, 171, 22, 190, 139, 25], OperandSize::Qword)
}

#[test]
fn vgetexpps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(YMM28)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 1948582555, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 125, 187, 66, 36, 93, 155, 2, 37, 116], OperandSize::Qword)
}

#[test]
fn vgetexpps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 154, 66, 192], OperandSize::Dword)
}

#[test]
fn vgetexpps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledDisplaced(EBX, Eight, 1129841744, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 203, 66, 4, 221, 80, 4, 88, 67], OperandSize::Dword)
}

#[test]
fn vgetexpps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexed(ESI, ESI, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 219, 66, 60, 246], OperandSize::Dword)
}

#[test]
fn vgetexpps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K3), broadcast: None }, &[98, 178, 125, 155, 66, 214], OperandSize::Qword)
}

#[test]
fn vgetexpps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexed(RAX, RAX, Eight, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 205, 66, 60, 192], OperandSize::Qword)
}

#[test]
fn vgetexpps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VGETEXPPS, operand1: Some(Direct(ZMM25)), operand2: Some(IndirectDisplaced(RDI, 367135497, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 125, 222, 66, 143, 9, 11, 226, 21], OperandSize::Qword)
}

