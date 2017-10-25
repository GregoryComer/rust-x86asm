use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmulhw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 229, 216], OperandSize::Dword)
}

#[test]
fn vpmulhw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 569106589, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 229, 4, 181, 157, 224, 235, 33], OperandSize::Dword)
}

#[test]
fn vpmulhw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 229, 239], OperandSize::Qword)
}

#[test]
fn vpmulhw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 229, 20, 210], OperandSize::Qword)
}

#[test]
fn vpmulhw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 229, 238], OperandSize::Dword)
}

#[test]
fn vpmulhw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Four, 837759547, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 229, 180, 190, 59, 50, 239, 49], OperandSize::Dword)
}

#[test]
fn vpmulhw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 229, 252], OperandSize::Qword)
}

#[test]
fn vpmulhw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 379118832, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 229, 20, 69, 240, 228, 152, 22], OperandSize::Qword)
}

#[test]
fn vpmulhw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 117, 142, 229, 240], OperandSize::Dword)
}

#[test]
fn vpmulhw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 1293354387, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 143, 229, 4, 85, 147, 5, 23, 77], OperandSize::Dword)
}

#[test]
fn vpmulhw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 65, 85, 132, 229, 244], OperandSize::Qword)
}

#[test]
fn vpmulhw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM31)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 920369018, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 5, 135, 229, 20, 205, 122, 183, 219, 54], OperandSize::Qword)
}

#[test]
fn vpmulhw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 77, 169, 229, 233], OperandSize::Dword)
}

#[test]
fn vpmulhw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Eight, 377581123, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 69, 173, 229, 148, 198, 67, 110, 129, 22], OperandSize::Dword)
}

#[test]
fn vpmulhw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 93, 170, 229, 203], OperandSize::Qword)
}

#[test]
fn vpmulhw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1589161205, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 167, 229, 12, 197, 245, 172, 184, 94], OperandSize::Qword)
}

#[test]
fn vpmulhw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 69, 201, 229, 218], OperandSize::Dword)
}

#[test]
fn vpmulhw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EBX, Two, 1274200973, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 101, 207, 229, 180, 91, 141, 195, 242, 75], OperandSize::Dword)
}

#[test]
fn vpmulhw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM13)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 21, 202, 229, 199], OperandSize::Qword)
}

#[test]
fn vpmulhw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMULHW, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Two, 854160571, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 45, 195, 229, 140, 82, 187, 116, 233, 50], OperandSize::Qword)
}

