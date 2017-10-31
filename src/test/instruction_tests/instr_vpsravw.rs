use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpsravw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 237, 138, 17, 237], OperandSize::Dword)
}

#[test]
fn vpsravw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectDisplaced(EBX, 476147397, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 245, 143, 17, 155, 197, 110, 97, 28], OperandSize::Dword)
}

#[test]
fn vpsravw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM24)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 130, 189, 134, 17, 198], OperandSize::Qword)
}

#[test]
fn vpsravw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 824030923, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 141, 131, 17, 60, 77, 203, 182, 29, 49], OperandSize::Qword)
}

#[test]
fn vpsravw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 221, 174, 17, 250], OperandSize::Dword)
}

#[test]
fn vpsravw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 245, 174, 17, 40], OperandSize::Dword)
}

#[test]
fn vpsravw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 18, 253, 171, 17, 250], OperandSize::Qword)
}

#[test]
fn vpsravw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 229, 174, 17, 57], OperandSize::Qword)
}

#[test]
fn vpsravw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 213, 202, 17, 234], OperandSize::Dword)
}

#[test]
fn vpsravw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Four, 1592830754, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 245, 201, 17, 180, 142, 34, 171, 240, 94], OperandSize::Dword)
}

#[test]
fn vpsravw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(ZMM30)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 2, 253, 202, 17, 240], OperandSize::Qword)
}

#[test]
fn vpsravw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSRAVW, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 1685792606, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 197, 205, 17, 12, 125, 94, 39, 123, 100], OperandSize::Qword)
}

