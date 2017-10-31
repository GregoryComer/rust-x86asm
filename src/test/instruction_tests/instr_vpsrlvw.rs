use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsrlvw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 229, 138, 16, 215], OperandSize::Dword)
}

#[test]
fn vpsrlvw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 140, 16, 27], OperandSize::Dword)
}

#[test]
fn vpsrlvw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 34, 253, 143, 16, 207], OperandSize::Qword)
}

#[test]
fn vpsrlvw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 165, 129, 16, 44, 240], OperandSize::Qword)
}

#[test]
fn vpsrlvw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 205, 174, 16, 199], OperandSize::Dword)
}

#[test]
fn vpsrlvw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 205, 171, 16, 10], OperandSize::Dword)
}

#[test]
fn vpsrlvw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM27)), operand3: Some(Direct(YMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 210, 165, 163, 16, 200], OperandSize::Qword)
}

#[test]
fn vpsrlvw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 189, 163, 16, 4, 74], OperandSize::Qword)
}

#[test]
fn vpsrlvw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 245, 201, 16, 232], OperandSize::Dword)
}

#[test]
fn vpsrlvw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Eight, 16614075, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 221, 206, 16, 172, 241, 187, 130, 253, 0], OperandSize::Dword)
}

#[test]
fn vpsrlvw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM28)), operand3: Some(Direct(ZMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 50, 157, 196, 16, 228], OperandSize::Qword)
}

#[test]
fn vpsrlvw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRLVW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Eight, 1826557727, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 157, 198, 16, 132, 206, 31, 15, 223, 108], OperandSize::Qword)
}

