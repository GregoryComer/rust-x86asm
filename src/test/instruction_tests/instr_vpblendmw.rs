use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpblendmw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 221, 137, 102, 229], OperandSize::Dword)
}

#[test]
fn vpblendmw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 1856928176, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 221, 139, 102, 52, 85, 176, 121, 174, 110], OperandSize::Dword)
}

#[test]
fn vpblendmw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 213, 143, 102, 197], OperandSize::Qword)
}

#[test]
fn vpblendmw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM20)), operand3: Some(Indirect(RDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 221, 133, 102, 42], OperandSize::Qword)
}

#[test]
fn vpblendmw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 237, 170, 102, 224], OperandSize::Dword)
}

#[test]
fn vpblendmw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 245, 170, 102, 30], OperandSize::Dword)
}

#[test]
fn vpblendmw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM24)), operand3: Some(Direct(YMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 210, 189, 161, 102, 238], OperandSize::Qword)
}

#[test]
fn vpblendmw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM10)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 724942524, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 173, 175, 102, 12, 69, 188, 190, 53, 43], OperandSize::Qword)
}

#[test]
fn vpblendmw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 213, 204, 102, 243], OperandSize::Dword)
}

#[test]
fn vpblendmw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 229, 204, 102, 1], OperandSize::Dword)
}

#[test]
fn vpblendmw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM12)), operand3: Some(Direct(ZMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 18, 157, 205, 102, 222], OperandSize::Qword)
}

#[test]
fn vpblendmw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMW, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM21)), operand3: Some(IndirectScaledIndexed(RSI, RAX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 213, 196, 102, 36, 198], OperandSize::Qword)
}

