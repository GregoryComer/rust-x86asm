use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovdb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 137, 49, 246], OperandSize::Dword)
}

#[test]
fn vpmovdb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(IndirectScaledDisplaced(ESI, Four, 1130437392, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 49, 28, 181, 16, 27, 97, 67], OperandSize::Dword)
}

#[test]
fn vpmovdb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 126, 143, 49, 231], OperandSize::Qword)
}

#[test]
fn vpmovdb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand2: Some(Direct(XMM27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 8, 49, 30], OperandSize::Qword)
}

#[test]
fn vpmovdb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 126, 170, 49, 251], OperandSize::Dword)
}

#[test]
fn vpmovdb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(IndirectScaledDisplaced(ECX, Four, 982689393, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 49, 12, 141, 113, 166, 146, 58], OperandSize::Dword)
}

#[test]
fn vpmovdb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Direct(XMM28)), operand2: Some(Direct(YMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 130, 126, 171, 49, 204], OperandSize::Qword)
}

#[test]
fn vpmovdb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(IndirectScaledDisplaced(RBX, Four, 1730739496, Some(OperandSize::Qword), None)), operand2: Some(Direct(YMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 40, 49, 44, 157, 40, 253, 40, 103], OperandSize::Qword)
}

#[test]
fn vpmovdb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 126, 204, 49, 225], OperandSize::Dword)
}

#[test]
fn vpmovdb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 49, 35], OperandSize::Dword)
}

#[test]
fn vpmovdb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(Direct(XMM16)), operand2: Some(Direct(ZMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 162, 126, 202, 49, 248], OperandSize::Qword)
}

#[test]
fn vpmovdb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVDB, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Eight, 1749350167, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(ZMM31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 72, 49, 188, 193, 23, 247, 68, 104], OperandSize::Qword)
}

