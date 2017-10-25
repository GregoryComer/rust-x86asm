use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpcmpb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(59)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 77, 15, 63, 255, 59], OperandSize::Dword)
}

fn vpcmpb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Four, 1343259746, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(67)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 125, 15, 63, 172, 151, 98, 132, 16, 80, 67], OperandSize::Dword)
}

fn vpcmpb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM22)), operand4: Some(Literal8(50)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 179, 109, 14, 63, 238, 50], OperandSize::Qword)
}

fn vpcmpb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1428938346, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(78)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 5, 11, 63, 44, 197, 106, 222, 43, 85, 78], OperandSize::Qword)
}

fn vpcmpb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: Some(Literal8(0)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 117, 44, 63, 245, 0], OperandSize::Dword)
}

fn vpcmpb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EDX, EDI, Eight, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(97)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 125, 42, 63, 52, 250, 97], OperandSize::Dword)
}

fn vpcmpb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM26)), operand3: Some(Direct(YMM19)), operand4: Some(Literal8(50)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 179, 45, 34, 63, 227, 50], OperandSize::Qword)
}

fn vpcmpb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM31)), operand3: Some(IndirectScaledIndexed(RCX, RDX, Eight, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(52)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 5, 34, 63, 12, 209, 52], OperandSize::Qword)
}

fn vpcmpb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM5)), operand4: Some(Literal8(21)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 125, 74, 63, 205, 21], OperandSize::Dword)
}

fn vpcmpb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectDisplaced(EAX, 461648891, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(62)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 125, 73, 63, 144, 251, 51, 132, 27, 62], OperandSize::Dword)
}

fn vpcmpb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM18)), operand3: Some(Direct(ZMM19)), operand4: Some(Literal8(5)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 179, 109, 71, 63, 235, 5], OperandSize::Qword)
}

fn vpcmpb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPB, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM25)), operand3: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(43)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 53, 66, 63, 33, 43], OperandSize::Qword)
}

