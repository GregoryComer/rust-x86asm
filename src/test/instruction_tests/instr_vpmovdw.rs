use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovdw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 141, 51, 222], OperandSize::Dword)
}

#[test]
fn vpmovdw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Four, 870148324, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 51, 164, 182, 228, 104, 221, 51], OperandSize::Dword)
}

#[test]
fn vpmovdw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 50, 126, 139, 51, 219], OperandSize::Qword)
}

#[test]
fn vpmovdw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Eight, 71414270, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 51, 180, 195, 254, 177, 65, 4], OperandSize::Qword)
}

#[test]
fn vpmovdw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 173, 51, 206], OperandSize::Dword)
}

#[test]
fn vpmovdw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EAX, Four, 2070724533, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 51, 148, 134, 181, 191, 108, 123], OperandSize::Dword)
}

#[test]
fn vpmovdw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(YMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 126, 169, 51, 248], OperandSize::Qword)
}

#[test]
fn vpmovdw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 126, 40, 51, 18], OperandSize::Qword)
}

#[test]
fn vpmovdw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 206, 51, 225], OperandSize::Dword)
}

#[test]
fn vpmovdw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(IndirectScaledDisplaced(EDX, Four, 1567199956, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 51, 28, 149, 212, 146, 105, 93], OperandSize::Dword)
}

#[test]
fn vpmovdw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(Direct(YMM26)), operand2: Some(Direct(ZMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 2, 126, 204, 51, 250], OperandSize::Qword)
}

#[test]
fn vpmovdw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDW, operand1: Some(IndirectDisplaced(RAX, 1741785530, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 72, 51, 168, 186, 137, 209, 103], OperandSize::Qword)
}

