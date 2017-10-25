use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmaddubsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 4, 238], OperandSize::Dword)
}

fn vpmaddubsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 4, 38], OperandSize::Dword)
}

fn vpmaddubsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 4, 229], OperandSize::Qword)
}

fn vpmaddubsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 306132095, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 4, 60, 205, 127, 52, 63, 18], OperandSize::Qword)
}

fn vpmaddubsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 4, 254], OperandSize::Dword)
}

fn vpmaddubsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 4, 49], OperandSize::Dword)
}

fn vpmaddubsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 4, 255], OperandSize::Qword)
}

fn vpmaddubsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Eight, 162242832, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 4, 132, 195, 16, 161, 171, 9], OperandSize::Qword)
}

fn vpmaddubsw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 101, 141, 4, 210], OperandSize::Dword)
}

fn vpmaddubsw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 1974652831, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 101, 143, 4, 28, 85, 159, 207, 178, 117], OperandSize::Dword)
}

fn vpmaddubsw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 178, 93, 139, 4, 225], OperandSize::Qword)
}

fn vpmaddubsw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(RDI, 1384903706, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 85, 140, 4, 175, 26, 244, 139, 82], OperandSize::Qword)
}

fn vpmaddubsw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 170, 4, 212], OperandSize::Dword)
}

fn vpmaddubsw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 101, 175, 4, 28, 215], OperandSize::Dword)
}

fn vpmaddubsw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM10)), operand3: Some(Direct(YMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 146, 45, 175, 4, 213], OperandSize::Qword)
}

fn vpmaddubsw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 936614889, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 117, 172, 4, 28, 93, 233, 155, 211, 55], OperandSize::Qword)
}

fn vpmaddubsw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 69, 207, 4, 230], OperandSize::Dword)
}

fn vpmaddubsw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(EAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 109, 204, 4, 24], OperandSize::Dword)
}

fn vpmaddubsw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM18)), operand3: Some(Direct(ZMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 194, 109, 195, 4, 240], OperandSize::Qword)
}

fn vpmaddubsw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMADDUBSW, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM18)), operand3: Some(IndirectDisplaced(RDX, 984994468, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 109, 193, 4, 138, 164, 210, 181, 58], OperandSize::Qword)
}

