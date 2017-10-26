use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsravw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 197, 141, 17, 201], OperandSize::Dword)
}

#[test]
fn vpsravw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 919205626, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 143, 17, 156, 86, 250, 246, 201, 54], OperandSize::Dword)
}

#[test]
fn vpsravw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 133, 133, 17, 206], OperandSize::Qword)
}

#[test]
fn vpsravw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 1929968310, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 134, 17, 36, 221, 182, 250, 8, 115], OperandSize::Qword)
}

#[test]
fn vpsravw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 205, 171, 17, 254], OperandSize::Dword)
}

#[test]
fn vpsravw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 213, 172, 17, 33], OperandSize::Dword)
}

#[test]
fn vpsravw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM22)), operand3: Some(Direct(YMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 146, 205, 163, 17, 194], OperandSize::Qword)
}

#[test]
fn vpsravw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 221, 169, 17, 44, 217], OperandSize::Qword)
}

#[test]
fn vpsravw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 197, 201, 17, 236], OperandSize::Dword)
}

#[test]
fn vpsravw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(EDX, 1159978042, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 213, 203, 17, 154, 58, 220, 35, 69], OperandSize::Dword)
}

#[test]
fn vpsravw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM30)), operand3: Some(Direct(ZMM12)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 82, 141, 198, 17, 252], OperandSize::Qword)
}

#[test]
fn vpsravw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectDisplaced(RBX, 1359564499, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 245, 193, 17, 171, 211, 78, 9, 81], OperandSize::Qword)
}

