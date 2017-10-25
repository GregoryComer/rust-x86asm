use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 54, 248], OperandSize::Dword)
}

#[test]
fn vpermd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(ECX, 250396525, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 54, 169, 109, 191, 236, 14], OperandSize::Dword)
}

#[test]
fn vpermd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 54, 231], OperandSize::Qword)
}

#[test]
fn vpermd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Eight, 353627001, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 54, 172, 208, 121, 235, 19, 21], OperandSize::Qword)
}

#[test]
fn vpermd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 125, 171, 54, 254], OperandSize::Dword)
}

#[test]
fn vpermd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EDX, 595278564, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 117, 174, 54, 130, 228, 58, 123, 35], OperandSize::Dword)
}

#[test]
fn vpermd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 197179816, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 117, 185, 54, 132, 119, 168, 185, 192, 11], OperandSize::Dword)
}

#[test]
fn vpermd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM31)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 5, 163, 54, 229], OperandSize::Qword)
}

#[test]
fn vpermd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM21)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1258983841, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 85, 165, 54, 44, 197, 161, 145, 10, 75], OperandSize::Qword)
}

#[test]
fn vpermd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectScaledIndexed(RSI, RBX, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 61, 190, 54, 12, 158], OperandSize::Qword)
}

#[test]
fn vpermd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 109, 206, 54, 215], OperandSize::Dword)
}

#[test]
fn vpermd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Four, 812792311, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 109, 207, 54, 164, 179, 247, 57, 114, 48], OperandSize::Dword)
}

#[test]
fn vpermd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Four, 693047766, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 93, 221, 54, 164, 130, 214, 17, 79, 41], OperandSize::Dword)
}

#[test]
fn vpermd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM20)), operand3: Some(Direct(ZMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 2, 93, 199, 54, 211], OperandSize::Qword)
}

#[test]
fn vpermd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(RDI, 1971185565, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 109, 201, 54, 159, 157, 231, 125, 117], OperandSize::Qword)
}

#[test]
fn vpermd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM18)), operand3: Some(IndirectDisplaced(RCX, 484427331, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 109, 213, 54, 137, 67, 198, 223, 28], OperandSize::Qword)
}

