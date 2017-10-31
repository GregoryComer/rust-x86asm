use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpminub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 218, 242], OperandSize::Dword)
}

#[test]
fn vpminub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Four, 1445419767, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 218, 132, 134, 247, 90, 39, 86], OperandSize::Dword)
}

#[test]
fn vpminub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 218, 230], OperandSize::Qword)
}

#[test]
fn vpminub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 218, 48], OperandSize::Qword)
}

#[test]
fn vpminub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 218, 238], OperandSize::Dword)
}

#[test]
fn vpminub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(ESI, EDI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 218, 4, 254], OperandSize::Dword)
}

#[test]
fn vpminub_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 218, 248], OperandSize::Qword)
}

#[test]
fn vpminub_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 218, 60, 147], OperandSize::Qword)
}

#[test]
fn vpminub_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 101, 143, 218, 234], OperandSize::Dword)
}

#[test]
fn vpminub_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(ECX, 753290708, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 117, 143, 218, 169, 212, 77, 230, 44], OperandSize::Dword)
}

#[test]
fn vpminub_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM17)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 117, 131, 218, 214], OperandSize::Qword)
}

#[test]
fn vpminub_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 1900418106, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 5, 142, 218, 140, 190, 58, 20, 70, 113], OperandSize::Qword)
}

#[test]
fn vpminub_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 172, 218, 211], OperandSize::Dword)
}

#[test]
fn vpminub_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Eight, 1499714550, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 172, 218, 180, 246, 246, 211, 99, 89], OperandSize::Dword)
}

#[test]
fn vpminub_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(YMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 37, 162, 218, 252], OperandSize::Qword)
}

#[test]
fn vpminub_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectDisplaced(RDX, 690245263, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 117, 161, 218, 162, 143, 78, 36, 41], OperandSize::Qword)
}

#[test]
fn vpminub_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 207, 218, 223], OperandSize::Dword)
}

#[test]
fn vpminub_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(ECX, 1094837194, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 109, 205, 218, 129, 202, 227, 65, 65], OperandSize::Dword)
}

#[test]
fn vpminub_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM21)), operand3: Some(Direct(ZMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 145, 85, 198, 218, 231], OperandSize::Qword)
}

#[test]
fn vpminub_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUB, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM19)), operand3: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 101, 195, 218, 17], OperandSize::Qword)
}

