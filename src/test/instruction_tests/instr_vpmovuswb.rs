use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovuswb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 126, 142, 16, 224], OperandSize::Dword)
}

#[test]
fn vpmovuswb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(IndirectScaledDisplaced(EAX, Two, 1213389298, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 16, 44, 69, 242, 217, 82, 72], OperandSize::Dword)
}

#[test]
fn vpmovuswb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 18, 126, 139, 16, 221], OperandSize::Qword)
}

#[test]
fn vpmovuswb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(IndirectScaledIndexed(RDI, RAX, Eight, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 16, 44, 199], OperandSize::Qword)
}

#[test]
fn vpmovuswb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 126, 173, 16, 241], OperandSize::Dword)
}

#[test]
fn vpmovuswb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Two, 947053536, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 16, 180, 119, 224, 227, 114, 56], OperandSize::Dword)
}

#[test]
fn vpmovuswb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(Direct(XMM25)), operand2: Some(Direct(YMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 18, 126, 173, 16, 193], OperandSize::Qword)
}

#[test]
fn vpmovuswb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(IndirectDisplaced(RAX, 1833222632, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 16, 176, 232, 193, 68, 109], OperandSize::Qword)
}

#[test]
fn vpmovuswb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(Direct(YMM2)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 201, 16, 210], OperandSize::Dword)
}

#[test]
fn vpmovuswb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Four, 1126056139, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 16, 180, 138, 203, 64, 30, 67], OperandSize::Dword)
}

#[test]
fn vpmovuswb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(Direct(YMM26)), operand2: Some(Direct(ZMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 130, 126, 201, 16, 218], OperandSize::Qword)
}

#[test]
fn vpmovuswb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVUSWB, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Eight, 1688035649, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 114, 126, 72, 16, 188, 209, 65, 97, 157, 100], OperandSize::Qword)
}

