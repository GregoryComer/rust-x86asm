use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpcmpuw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM1)), operand4: Some(Literal8(65)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 237, 12, 62, 201, 65], OperandSize::Dword)
}

fn vpcmpuw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 2135531102, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(125)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 237, 13, 62, 44, 117, 94, 158, 73, 127, 125], OperandSize::Dword)
}

fn vpcmpuw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM17)), operand4: Some(Literal8(23)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 179, 237, 14, 62, 241, 23], OperandSize::Qword)
}

fn vpcmpuw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectDisplaced(RCX, 910108577, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(10)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 205, 14, 62, 177, 161, 39, 63, 54, 10], OperandSize::Qword)
}

fn vpcmpuw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: Some(Literal8(1)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 197, 47, 62, 209, 1], OperandSize::Dword)
}

fn vpcmpuw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 504258471, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(101)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 205, 44, 62, 28, 133, 167, 95, 14, 30, 101], OperandSize::Dword)
}

fn vpcmpuw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM21)), operand3: Some(Direct(YMM18)), operand4: Some(Literal8(63)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 179, 213, 33, 62, 210, 63], OperandSize::Qword)
}

fn vpcmpuw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Eight, 197049828, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(44)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 197, 42, 62, 180, 223, 228, 189, 190, 11, 44], OperandSize::Qword)
}

fn vpcmpuw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM0)), operand4: Some(Literal8(1)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 221, 78, 62, 200, 1], OperandSize::Dword)
}

fn vpcmpuw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexed(EDX, EBX, Four, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(40)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 213, 75, 62, 28, 154, 40], OperandSize::Dword)
}

fn vpcmpuw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM22)), operand3: Some(Direct(ZMM15)), operand4: Some(Literal8(48)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 211, 205, 68, 62, 223, 48], OperandSize::Qword)
}

fn vpcmpuw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUW, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectScaledIndexed(RAX, RAX, Four, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(34)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 245, 71, 62, 52, 128, 34], OperandSize::Qword)
}

