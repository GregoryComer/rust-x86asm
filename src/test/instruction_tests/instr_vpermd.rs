use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 54, 227], OperandSize::Dword)
}

#[test]
fn vpermd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 1837032182, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 54, 44, 205, 246, 226, 126, 109], OperandSize::Dword)
}

#[test]
fn vpermd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 54, 231], OperandSize::Qword)
}

#[test]
fn vpermd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(RDX, 665500267, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 54, 138, 107, 186, 170, 39], OperandSize::Qword)
}

#[test]
fn vpermd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 77, 173, 54, 237], OperandSize::Dword)
}

#[test]
fn vpermd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(ESI, 781051548, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 85, 169, 54, 158, 156, 230, 141, 46], OperandSize::Dword)
}

#[test]
fn vpermd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 149591087, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 109, 186, 54, 52, 189, 47, 148, 234, 8], OperandSize::Dword)
}

#[test]
fn vpermd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM8)), operand3: Some(Direct(YMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 2, 61, 170, 54, 225], OperandSize::Qword)
}

#[test]
fn vpermd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 1067411581, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 45, 162, 54, 60, 221, 125, 104, 159, 63], OperandSize::Qword)
}

#[test]
fn vpermd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(YMM22)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectDisplaced(RSI, 1049602403, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 45, 182, 54, 182, 99, 169, 143, 62], OperandSize::Qword)
}

#[test]
fn vpermd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 125, 207, 54, 207], OperandSize::Dword)
}

#[test]
fn vpermd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(EDI, 892491531, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 93, 202, 54, 191, 11, 87, 50, 53], OperandSize::Dword)
}

#[test]
fn vpermd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(ECX, 261810386, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 125, 217, 54, 185, 210, 232, 154, 15], OperandSize::Dword)
}

#[test]
fn vpermd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM22)), operand3: Some(Direct(ZMM18)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 34, 77, 193, 54, 234], OperandSize::Qword)
}

#[test]
fn vpermd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 101, 206, 54, 2], OperandSize::Qword)
}

#[test]
fn vpermd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMD, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 2017989298, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 101, 221, 54, 4, 69, 178, 18, 72, 120], OperandSize::Qword)
}

