use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsravw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 197, 143, 17, 238], OperandSize::Dword)
}

#[test]
fn vpsravw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 245, 138, 17, 31], OperandSize::Dword)
}

#[test]
fn vpsravw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM30)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 146, 141, 129, 17, 232], OperandSize::Qword)
}

#[test]
fn vpsravw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1392237660, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 229, 139, 17, 36, 181, 92, 220, 251, 82], OperandSize::Qword)
}

#[test]
fn vpsravw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 237, 175, 17, 208], OperandSize::Dword)
}

#[test]
fn vpsravw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 229, 171, 17, 20, 129], OperandSize::Dword)
}

#[test]
fn vpsravw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 194, 245, 169, 17, 208], OperandSize::Qword)
}

#[test]
fn vpsravw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 921348683, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 141, 163, 17, 4, 93, 75, 170, 234, 54], OperandSize::Qword)
}

#[test]
fn vpsravw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 245, 201, 17, 218], OperandSize::Dword)
}

#[test]
fn vpsravw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 89965666, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 197, 201, 17, 4, 205, 98, 196, 92, 5], OperandSize::Dword)
}

#[test]
fn vpsravw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM28)), operand3: Some(Direct(ZMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 194, 157, 199, 17, 234], OperandSize::Qword)
}

#[test]
fn vpsravw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 237, 201, 17, 18], OperandSize::Qword)
}

