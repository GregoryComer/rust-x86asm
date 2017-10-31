use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vprorvq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 245, 142, 20, 226], OperandSize::Dword)
}

#[test]
fn vprorvq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EBX, ESI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 205, 142, 20, 12, 179], OperandSize::Dword)
}

#[test]
fn vprorvq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 213, 157, 20, 28, 151], OperandSize::Dword)
}

#[test]
fn vprorvq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 146, 205, 142, 20, 208], OperandSize::Qword)
}

#[test]
fn vprorvq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 419000431, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 149, 134, 20, 20, 157, 111, 112, 249, 24], OperandSize::Qword)
}

#[test]
fn vprorvq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectScaledIndexed(RDI, RDI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 173, 146, 20, 36, 127], OperandSize::Qword)
}

#[test]
fn vprorvq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 245, 171, 20, 231], OperandSize::Dword)
}

#[test]
fn vprorvq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EAX, 1437124397, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 221, 171, 20, 168, 45, 199, 168, 85], OperandSize::Dword)
}

#[test]
fn vprorvq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Eight, 550079838, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 197, 191, 20, 172, 218, 94, 141, 201, 32], OperandSize::Dword)
}

#[test]
fn vprorvq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 2, 253, 174, 20, 250], OperandSize::Qword)
}

#[test]
fn vprorvq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 245, 166, 20, 28, 211], OperandSize::Qword)
}

#[test]
fn vprorvq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 61455857, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 173, 177, 20, 44, 221, 241, 189, 169, 3], OperandSize::Qword)
}

#[test]
fn vprorvq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 229, 205, 20, 224], OperandSize::Dword)
}

#[test]
fn vprorvq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(ESI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 229, 202, 20, 14], OperandSize::Dword)
}

#[test]
fn vprorvq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 229, 221, 20, 6], OperandSize::Dword)
}

#[test]
fn vprorvq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 50, 229, 206, 20, 255], OperandSize::Qword)
}

#[test]
fn vprorvq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM23)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Four, 1953150745, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 197, 193, 20, 180, 144, 25, 183, 106, 116], OperandSize::Qword)
}

#[test]
fn vprorvq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPRORVQ, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1971157642, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 205, 214, 20, 60, 189, 138, 122, 125, 117], OperandSize::Qword)
}

