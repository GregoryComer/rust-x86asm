use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpcmpeqw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 117, 207], OperandSize::Dword)
}

fn vpcmpeqw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EAX, Eight, 1166975377, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 117, 148, 199, 145, 161, 142, 69], OperandSize::Dword)
}

fn vpcmpeqw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 117, 222], OperandSize::Qword)
}

fn vpcmpeqw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(RDX, 1471867440, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 225, 117, 178, 48, 234, 186, 87], OperandSize::Qword)
}

fn vpcmpeqw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 117, 239], OperandSize::Dword)
}

fn vpcmpeqw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 117, 49], OperandSize::Dword)
}

fn vpcmpeqw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 245, 117, 251], OperandSize::Qword)
}

fn vpcmpeqw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 252489456, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 117, 36, 157, 240, 174, 12, 15], OperandSize::Qword)
}

fn vpcmpeqw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 109, 12, 117, 247], OperandSize::Dword)
}

fn vpcmpeqw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 1100933467, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 85, 15, 117, 44, 157, 91, 233, 158, 65], OperandSize::Dword)
}

fn vpcmpeqw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K7)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 145, 93, 6, 117, 253], OperandSize::Qword)
}

fn vpcmpeqw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Four, 271092567, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 10, 117, 140, 186, 87, 139, 40, 16], OperandSize::Qword)
}

fn vpcmpeqw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 42, 117, 244], OperandSize::Dword)
}

fn vpcmpeqw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K6)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 1073138513, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 69, 46, 117, 180, 86, 81, 203, 246, 63], OperandSize::Dword)
}

fn vpcmpeqw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM25)), operand3: Some(Direct(YMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 145, 53, 37, 117, 230], OperandSize::Qword)
}

fn vpcmpeqw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Four, 1771478510, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 29, 47, 117, 164, 143, 238, 157, 150, 105], OperandSize::Qword)
}

fn vpcmpeqw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM4)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 93, 78, 117, 226], OperandSize::Dword)
}

fn vpcmpeqw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 77, 75, 117, 44, 129], OperandSize::Dword)
}

fn vpcmpeqw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K6)), operand2: Some(Direct(ZMM9)), operand3: Some(Direct(ZMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 209, 53, 79, 117, 241], OperandSize::Qword)
}

fn vpcmpeqw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPEQW, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 109, 75, 117, 36, 247], OperandSize::Qword)
}

