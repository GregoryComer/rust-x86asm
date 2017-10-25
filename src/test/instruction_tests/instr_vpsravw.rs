use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsravw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 229, 143, 17, 201], OperandSize::Dword)
}

#[test]
fn vpsravw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(EBX, 325010616, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 140, 17, 139, 184, 68, 95, 19], OperandSize::Dword)
}

#[test]
fn vpsravw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 162, 157, 135, 17, 240], OperandSize::Qword)
}

#[test]
fn vpsravw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM27)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Eight, 1225905593, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 165, 134, 17, 148, 195, 185, 213, 17, 73], OperandSize::Qword)
}

#[test]
fn vpsravw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 221, 169, 17, 200], OperandSize::Dword)
}

#[test]
fn vpsravw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(EAX, 183559671, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 245, 175, 17, 168, 247, 229, 240, 10], OperandSize::Dword)
}

#[test]
fn vpsravw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 66, 197, 173, 17, 249], OperandSize::Qword)
}

#[test]
fn vpsravw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectDisplaced(RBX, 1319058426, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 181, 171, 17, 139, 250, 59, 159, 78], OperandSize::Qword)
}

#[test]
fn vpsravw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 237, 206, 17, 219], OperandSize::Dword)
}

#[test]
fn vpsravw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 1549057676, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 221, 201, 17, 140, 86, 140, 190, 84, 92], OperandSize::Dword)
}

#[test]
fn vpsravw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 18, 189, 195, 17, 243], OperandSize::Qword)
}

#[test]
fn vpsravw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 245, 204, 17, 24], OperandSize::Qword)
}

