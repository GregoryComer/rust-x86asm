use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpcmpub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(17)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 109, 14, 62, 203, 17], OperandSize::Dword)
}

fn vpcmpub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Four, 27899211, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(56)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 101, 10, 62, 148, 152, 75, 181, 169, 1, 56], OperandSize::Dword)
}

fn vpcmpub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM14)), operand4: Some(Literal8(52)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 211, 109, 10, 62, 214, 52], OperandSize::Qword)
}

fn vpcmpub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1131204265, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(93)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 101, 3, 62, 20, 117, 169, 206, 108, 67, 93], OperandSize::Qword)
}

fn vpcmpub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM4)), operand4: Some(Literal8(14)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 93, 44, 62, 252, 14], OperandSize::Dword)
}

fn vpcmpub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Two, 2073979736, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(0)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 101, 42, 62, 164, 127, 88, 107, 158, 123, 0], OperandSize::Dword)
}

fn vpcmpub_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(YMM31)), operand4: Some(Literal8(15)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 147, 5, 45, 62, 215, 15], OperandSize::Qword)
}

fn vpcmpub_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM19)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(43)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 101, 37, 62, 42, 43], OperandSize::Qword)
}

fn vpcmpub_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM3)), operand4: Some(Literal8(119)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 69, 73, 62, 203, 119], OperandSize::Dword)
}

fn vpcmpub_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 996199483, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(88)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 117, 79, 62, 12, 253, 59, 204, 96, 59, 88], OperandSize::Dword)
}

fn vpcmpub_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM12)), operand3: Some(Direct(ZMM22)), operand4: Some(Literal8(113)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 179, 29, 73, 62, 254, 113], OperandSize::Qword)
}

fn vpcmpub_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPUB, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM20)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 479754913, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(87)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 93, 71, 62, 60, 189, 161, 122, 152, 28, 87], OperandSize::Qword)
}

